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
