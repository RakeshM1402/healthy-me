use healthifyme_mobile::{MobileAppState, get_workout_schemas, AppUpdater};

fn main() {
    println!("=== HealthifyMe Mobile App (Rust Native Core) ===");

    // Initialize In-App Update Checker pointing to RakeshM1402/healthy-me
    let updater = AppUpdater::default_repo();
    println!("Checking for updates from https://github.com/{}/{}...", updater.repo_owner, updater.repo_name);
    match updater.check_latest_release() {
        Ok(Some((tag, url))) => println!("-> New update available: {} ({})", tag, url),
        Ok(None) => println!("-> App is up to date (version {})", updater.current_version),
        Err(e) => println!("-> Update check skipped / offline ({})", e),
    }

    let mut state = MobileAppState::new();
    println!("Initial Daily Calorie Target (TDEE): {:.1} kcal", state.daily_calorie_target);

    // Simulate Diary Food Log lookup
    println!("\n--- Diary Food Log Lookup ---");
    if let Some(item) = state.add_food_to_diary("Dal Tadka") {
        println!("Added food to log: {} ({} kcal)", item.name, item.calories);
    }
    if let Some(item) = state.add_food_to_diary("Roti") {
        println!("Added food to log: {} ({} kcal)", item.name, item.calories);
    }

    let consumed = state.get_consumed_calories();
    let progress = state.progress_percentage();
    println!("Dashboard Progress: {:.1} / {:.1} kcal ({:.1}%)", consumed, state.daily_calorie_target, progress);

    // Simulate Workout Registry grids
    println!("\n--- Workout Registry Grids ---");
    let workouts = get_workout_schemas();
    for w in workouts {
        println!("- [{}] {} ({}, {} sets of {})", w.category, w.name, w.muscle_group, w.default_sets, w.default_reps);
    }

    println!("\nMobile architecture setup and core translation completed successfully.");
}
