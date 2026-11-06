pub mod food;
pub mod workout;
pub mod calculator;
pub mod state;
pub mod updater;

pub use food::{FoodItem, get_food_database, search_food};
pub use workout::{Exercise, get_workout_schemas, filter_workouts_by_category};
pub use calculator::{ProfileCalculator, Gender, ActivityLevel};
pub use state::MobileAppState;
pub use updater::AppUpdater;

pub fn version() -> &'static str {
    "0.1.0-mobile"
}

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn android_main(app: android_activity::AndroidApp) {
    let _ = env_logger::builder().try_init();
    log::info!("HealthyMe Android app started");

    let state = MobileAppState::new();
    log::info!("Initialized MobileAppState with TDEE: {}", state.daily_calorie_target);

    loop {
        app.poll_events(Some(std::time::Duration::from_millis(100)), |_event| {
            // Handle Android lifecycle / input events
        });
    }
}
