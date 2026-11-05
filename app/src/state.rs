use crate::food::{FoodItem, search_food};
use crate::workout::{Exercise, get_workout_schemas};
use crate::calculator::{ProfileCalculator, Gender, ActivityLevel};

pub struct MobileAppState {
    pub user_profile: ProfileCalculator,
    pub food_log: Vec<FoodItem>,
    pub workout_registry: Vec<Exercise>,
    pub daily_calorie_target: f32,
}

impl MobileAppState {
    pub fn new() -> Self {
        let profile = ProfileCalculator::new(70.0, 175.0, 28, Gender::Male, ActivityLevel::Moderate);
        let (tdee, _, _, _) = profile.calculate_macros();

        Self {
            user_profile: profile,
            food_log: vec![],
            workout_registry: get_workout_schemas(),
            daily_calorie_target: tdee,
        }
    }

    pub fn add_food_to_diary(&mut self, query: &str) -> Option<FoodItem> {
        let results = search_food(query);
        if let Some(item) = results.into_iter().next() {
            self.food_log.push(item.clone());
            Some(item)
        } else {
            None
        }
    }

    pub fn get_consumed_calories(&self) -> f32 {
        self.food_log.iter().map(|f| f.calories).sum()
    }

    pub fn get_consumed_macros(&self) -> (f32, f32, f32) {
        let protein = self.food_log.iter().map(|f| f.protein).sum();
        let carbs = self.food_log.iter().map(|f| f.carbs).sum();
        let fat = self.food_log.iter().map(|f| f.fat).sum();
        (protein, carbs, fat)
    }

    pub fn progress_percentage(&self) -> f32 {
        let consumed = self.get_consumed_calories();
        if self.daily_calorie_target > 0.0 {
            (consumed / self.daily_calorie_target) * 100.0
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mobile_app_state() {
        let mut state = MobileAppState::new();
        assert_eq!(state.food_log.len(), 0);

        let added = state.add_food_to_diary("Roti");
        assert!(added.is_some());
        assert_eq!(state.food_log.len(), 1);
        assert!(state.get_consumed_calories() > 0.0);
    }
}
