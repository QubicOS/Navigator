use chrono::Local;
use slint::{ComponentHandle, Timer, TimerMode};
use std::time::Duration;

use crate::time::{format_date, format_time};
use crate::AppWindow;

// Build the window, wire callbacks, and start timers.
pub fn run_app() -> Result<(), slint::PlatformError> {
    let app = AppWindow::new()?;
    let weak = app.as_weak();

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
