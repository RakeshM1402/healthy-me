use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exercise {
    pub id: u32,
    pub name: &'static str,
    pub category: &'static str,
    pub muscle_group: &'static str,
    pub default_sets: u32,
    pub default_reps: u32,
    pub default_duration_secs: u32,
    pub calories_per_min: f32,
}

pub fn get_workout_schemas() -> Vec<Exercise> {
    vec![
        Exercise {
            id: 1,
            name: "Full Body HIIT Blast",
            category: "Cardio / HIIT",
            muscle_group: "Full Body",
            default_sets: 4,
            default_reps: 15,
            default_duration_secs: 1200,
            calories_per_min: 10.5,
        },
        Exercise {
            id: 2,
            name: "Vinyasa Flow Yoga",
            category: "Yoga & Flexibility",
            muscle_group: "Mobility & Core",
            default_sets: 1,
            default_reps: 1,
            default_duration_secs: 1800,
            calories_per_min: 4.0,
        },
        Exercise {
            id: 3,
            name: "Upper Body Strength (Push-ups & Dips)",
            category: "Strength Training",
            muscle_group: "Chest, Triceps, Shoulders",
            default_sets: 3,
            default_reps: 12,
            default_duration_secs: 900,
            calories_per_min: 7.0,
        },
        Exercise {
            id: 4,
            name: "Core Crusher (Planks & Crunches)",
            category: "Core Workout",
            muscle_group: "Abdominals",
            default_sets: 3,
            default_reps: 20,
            default_duration_secs: 600,
            calories_per_min: 6.5,
        },
        Exercise {
            id: 5,
            name: "Brisk Outdoor Running",
            category: "Cardio",
            muscle_group: "Lower Body",
            default_sets: 1,
            default_reps: 1,
            default_duration_secs: 1800,
            calories_per_min: 11.2,
        },
        Exercise {
            id: 6,
            name: "Cycling (Stationary / Outdoor)",
            category: "Cardio",
            muscle_group: "Legs & Glutes",
            default_sets: 1,
            default_reps: 1,
            default_duration_secs: 1800,
            calories_per_min: 9.0,
        },
    ]
}

pub fn filter_workouts_by_category(cat: &str) -> Vec<Exercise> {
    let db = get_workout_schemas();
    let c = cat.to_lowercase();
    db.into_iter()
        .filter(|ex| ex.category.to_lowercase().contains(&c))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workout_filter() {
        let results = filter_workouts_by_category("cardio");
        assert!(!results.is_empty());
    }
}
