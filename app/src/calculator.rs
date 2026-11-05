use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActivityLevel {
    Sedentary,     // Little or no exercise
    Light,         // Light exercise 1-3 days/week
    Moderate,      // Moderate exercise 3-5 days/week
    Active,        // Heavy exercise 6-7 days/week
    VeryActive,    // Very heavy exercise, physical job
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileCalculator {
    pub weight_kg: f32,
    pub height_cm: f32,
    pub age: u32,
    pub gender: Gender,
    pub activity_level: ActivityLevel,
}

impl ProfileCalculator {
    pub fn new(weight_kg: f32, height_cm: f32, age: u32, gender: Gender, activity_level: ActivityLevel) -> Self {
        Self {
            weight_kg,
            height_cm,
            age,
            gender,
            activity_level,
        }
    }

    /// Mifflin-St Jeor BMR Equation
    pub fn calculate_bmr(&self) -> f32 {
        let base = (10.0 * self.weight_kg) + (6.25 * self.height_cm) - (5.0 * self.age as f32);
        match self.gender {
            Gender::Male => base + 5.0,
            Gender::Female => base - 161.0,
        }
    }

    /// Total Daily Energy Expenditure (TDEE)
    pub fn calculate_tdee(&self) -> f32 {
        let bmr = self.calculate_bmr();
        let multiplier = match self.activity_level {
            ActivityLevel::Sedentary => 1.2,
            ActivityLevel::Light => 1.375,
            ActivityLevel::Moderate => 1.55,
            ActivityLevel::Active => 1.725,
            ActivityLevel::VeryActive => 1.9,
        };
        bmr * multiplier
    }

    /// Recommended Macros for Maintenance/Goal (Protein 30%, Carbs 45%, Fat 25%)
    pub fn calculate_macros(&self) -> (f32, f32, f32, f32) {
        let tdee = self.calculate_tdee();
        let protein_cals = tdee * 0.30;
        let carb_cals = tdee * 0.45;
        let fat_cals = tdee * 0.25;

        let protein_g = protein_cals / 4.0;
        let carbs_g = carb_cals / 4.0;
        let fat_g = fat_cals / 9.0;

        (tdee, protein_g, carbs_g, fat_g)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mifflin_st_jeor() {
        let calc = ProfileCalculator::new(70.0, 175.0, 30, Gender::Male, ActivityLevel::Moderate);
        let bmr = calc.calculate_bmr();
        assert!(bmr > 1500.0 && bmr < 1800.0);
        let tdee = calc.calculate_tdee();
        assert!(tdee > bmr);
    }
}
