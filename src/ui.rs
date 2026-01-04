use chrono::Local;
use slint::{ComponentHandle, Timer, TimerMode};
use std::time::Duration;

use crate::time::{format_date, format_time};
use crate::AppWindow;

fn reduce_motion_requested() -> bool {
    let env_set = std::env::var("BB_REDUCE_MOTION")
        .ok()
        .is_some_and(|value| matches!(value.as_str(), "1" | "true" | "TRUE" | "yes" | "YES"));

    let arg_set = std::env::args().any(|arg| matches!(arg.as_str(), "--reduce-motion" | "--no-animations"));

    env_set || arg_set
}

fn normalize_app_id(raw: &str) -> &str {
    match raw {
        "settings" | "Настройки" => "settings",
        "terminal" | "Терминал" => "terminal",
        "apps" | "Приложения" => "apps",
        other => other,
    }
}

// Build the window, wire callbacks, and start timers.
pub fn run_app() -> Result<(), slint::PlatformError> {
    let app = AppWindow::new()?;
    let weak = app.as_weak();

    let reduce_motion = reduce_motion_requested();
    app.set_reduce_motion(reduce_motion);
    app.set_app_version(env!("CARGO_PKG_VERSION").into());
    app.set_runtime_os(std::env::consts::OS.into());
    app.set_runtime_arch(std::env::consts::ARCH.into());
    app.set_runtime_platform(format!("{}/{}", std::env::consts::OS, std::env::consts::ARCH).into());

    // Keep the clock text in sync with system time.
    let update_time = move || {
        if let Some(app) = weak.upgrade() {
            let now = Local::now();
            let time_text = format_time(now);
            let date_text = format_date(now);
            app.set_time_text(time_text.into());
            app.set_date_text(date_text.into());
        }
    };

    update_time();

    let timer = Timer::default();
    timer.start(TimerMode::Repeated, Duration::from_secs(1), update_time);

    // Drive the lock-screen shimmer (disabled when reduce_motion=true).
    let shimmer_timer = Timer::default();
    if !reduce_motion {
        let shimmer_weak = app.as_weak();
        shimmer_timer.start(
            TimerMode::Repeated,
            Duration::from_millis(1800),
            move || {
                if let Some(app) = shimmer_weak.upgrade() {
                    let next = app.get_lock_shimmer_phase().wrapping_add(1);
                    app.set_lock_shimmer_phase(next);
                }
            },
        );
    }

    // Show a short-lived toast when an app tile is activated.
    let toast_timer = Timer::default();
    let toast_weak = app.as_weak();
    app.on_open_app(move |label| {
        if let Some(app) = toast_weak.upgrade() {
            let app_id = normalize_app_id(&label);
            app.set_shade_open(false);
            app.set_current_app_id(app_id.into());
            app.set_app_open(true);

            let text = format!("Открыто: {}", label);
            app.set_toast_text(text.into());
            app.set_toast_visible(true);
            let app_weak = app.as_weak();
            toast_timer.start(TimerMode::SingleShot, Duration::from_secs(1), move || {
                if let Some(app) = app_weak.upgrade() {
                    app.set_toast_visible(false);
                }
            });
        }
    });

    app.run()
}
