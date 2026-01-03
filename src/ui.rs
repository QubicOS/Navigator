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

// Build the window, wire callbacks, and start timers.
pub fn run_app() -> Result<(), slint::PlatformError> {
    let app = AppWindow::new()?;
    let weak = app.as_weak();

    let reduce_motion = reduce_motion_requested();
    app.set_reduce_motion(reduce_motion);

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
            let text = format!("Открыто: {}", label);
            app.set_toast_text(text.into());
            app.set_toast_visible(true);
            let app_weak = app.as_weak();
            toast_timer.start(TimerMode::SingleShot, Duration::from_secs(2), move || {
                if let Some(app) = app_weak.upgrade() {
                    app.set_toast_visible(false);
                }
            });
        }
    });

    app.run()
}
