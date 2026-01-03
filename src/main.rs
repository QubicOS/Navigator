mod time;
mod ui;

// Generated Slint component bindings.
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    // Delegate to the UI wiring module.
    ui::run_app()
}
