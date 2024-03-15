// tests/count_models_test.rs
use btlz_rust::view_models;
use std::error::Error;

#[test]
fn test_model_count() -> Result<(), Box<dyn Error>> {
    let (classes, weapons, battles) = view_models::view_models()?;

    // Dynamically determine the expected counts (for the sake of example, hard-coded here)
    let expected_classes_count = 5;
    let expected_weapons_count = 5;
    let expected_battles_count = 5;

    // Actual counts
    let actual_classes_count = classes.len();
    let actual_weapons_count = weapons.len();
    let actual_battles_count = battles.len();

    // Assertions
    assert_eq!(actual_classes_count, expected_classes_count, "Mismatch in classes count");
    assert_eq!(actual_weapons_count, expected_weapons_count, "Mismatch in weapons count");
    assert_eq!(actual_battles_count, expected_battles_count, "Mismatch in battles count");

    // Pretty print each class, weapon, and battle
    // (Your existing code for printing details goes here)

    // Print emoji-based results
    println!("{}", emoji_result("Classes", expected_classes_count, actual_classes_count));
    println!("{}", emoji_result("Weapons", expected_weapons_count, actual_weapons_count));
    println!("{}", emoji_result("Battles", expected_battles_count, actual_battles_count));

    Ok(())
}

fn emoji_result(model: &str, expected: usize, actual: usize) -> String {
    if expected == actual {
        format!("✅ Expected {} {}. Loaded {} {}.", expected, model, actual, model)
    } else {
        format!("❌ Expected {} {}. Loaded {} {}.", expected, model, actual, model)
    }
}
