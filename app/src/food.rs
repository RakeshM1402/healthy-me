use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodItem {
    pub id: u32,
    pub name: &'static str,
    pub category: &'static str,
    pub serving_size: &'static str,
    pub calories: f32,
    pub protein: f32,
    pub carbs: f32,
    pub fat: f32,
    pub fiber: f32,
}

pub fn get_food_database() -> Vec<FoodItem> {
    vec![
        FoodItem {
            id: 1,
            name: "Roti / Chapati (Plain, Wheat)",
            category: "Grains & Breads",
            serving_size: "1 medium (30g)",
            calories: 104.0,
            protein: 3.0,
            carbs: 18.0,
            fat: 2.2,
            fiber: 2.0,
        },
        FoodItem {
            id: 2,
            name: "Dal Tadka (Yellow Lentils)",
            category: "Lentils & Dals",
            serving_size: "1 bowl (150g)",
            calories: 145.0,
            protein: 8.5,
            carbs: 20.0,
            fat: 3.5,
            fiber: 5.0,
        },
        FoodItem {
            id: 3,
            name: "Paneer Butter Masala",
            category: "Main Course",
            serving_size: "1 serving (200g)",
            calories: 320.0,
            protein: 12.0,
            carbs: 14.0,
            fat: 24.0,
            fiber: 2.1,
        },
        FoodItem {
            id: 4,
            name: "Chicken Biryani (Hyderabadi)",
            category: "Rice & Biryani",
            serving_size: "1 plate (300g)",
            calories: 450.0,
            protein: 22.0,
            carbs: 55.0,
            fat: 16.0,
            fiber: 3.0,
        },
        FoodItem {
            id: 5,
            name: "Idli (South Indian Rice Cake)",
            category: "Breakfast",
            serving_size: "2 pieces (80g)",
            calories: 78.0,
            protein: 2.0,
            carbs: 16.5,
            fat: 0.4,
            fiber: 1.0,
        },
        FoodItem {
            id: 6,
            name: "Masala Dosa",
            category: "Breakfast",
            serving_size: "1 medium (150g)",
            calories: 250.0,
            protein: 4.5,
            carbs: 35.0,
            fat: 10.0,
            fiber: 2.5,
        },
        FoodItem {
            id: 7,
            name: "Poha (Flattened Rice)",
            category: "Breakfast",
            serving_size: "1 bowl (150g)",
            calories: 180.0,
            protein: 3.2,
            carbs: 30.0,
            fat: 5.5,
            fiber: 2.0,
        },
        FoodItem {
            id: 8,
            name: "Brown Rice (Cooked)",
            category: "Grains",
            serving_size: "1 cup (195g)",
            calories: 218.0,
            protein: 4.5,
            carbs: 45.8,
            fat: 1.6,
            fiber: 3.5,
        },
        FoodItem {
            id: 9,
            name: "Oats Porridge (with Milk)",
            category: "Breakfast",
            serving_size: "1 bowl (250g)",
            calories: 210.0,
            protein: 8.0,
            carbs: 32.0,
            fat: 5.0,
            fiber: 4.2,
        },
        FoodItem {
            id: 10,
            name: "Greek Yogurt (Plain, Low Fat)",
            category: "Dairy",
            serving_size: "1 container (170g)",
            calories: 100.0,
            protein: 17.0,
            carbs: 6.0,
            fat: 0.7,
            fiber: 0.0,
        },
        FoodItem {
            id: 11,
            name: "Banana (Fresh)",
            category: "Fruits",
            serving_size: "1 medium (118g)",
            calories: 105.0,
            protein: 1.3,
            carbs: 27.0,
            fat: 0.3,
            fiber: 3.1,
        },
        FoodItem {
            id: 12,
            name: "Almonds (Raw)",
            category: "Nuts & Seeds",
            serving_size: "1 handful (28g)",
            calories: 164.0,
            protein: 6.0,
            carbs: 6.0,
            fat: 14.0,
            fiber: 3.5,
        },
    ]
}

pub fn search_food(query: &str) -> Vec<FoodItem> {
    let db = get_food_database();
    let q = query.to_lowercase();
    db.into_iter()
        .filter(|item| item.name.to_lowercase().contains(&q) || item.category.to_lowercase().contains(&q))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_food_search() {
        let results = search_food("Roti");
        assert!(!results.is_empty());
        assert_eq!(results[0].name, "Roti / Chapati (Plain, Wheat)");
    }
}
