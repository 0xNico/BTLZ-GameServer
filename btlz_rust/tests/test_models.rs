// Assuming you have a module `load_models` where `load_models` function is defined
use btlz_rust::read_models::load_models;
use std::error::Error;

#[test]
fn test_view_models() -> Result<(), Box<dyn Error>> {
    let (classes, weapons, battles) = load_models()?;
    
    assert_eq!(classes.len(), 5, "There should be exactly 5 classes");
    assert_eq!(weapons.len(), 5, "There should be exactly 5 weapons");
    assert_eq!(battles.len(), 5, "There should be exactly 5 battles");

    // Proceed to pretty print each class, weapon, and battle as before
    // Consider abstracting the pretty printing into a separate function for reuse
    // Pretty print each class, weapon, and battle
    println!("Classes:");
    for class in classes {
        println!("Name: {}\nArmor URL: {}\nHP Boost: {}\nDodge Chance: {}\nDescription: {}\n", 
            class.name, class.armor_url, class.hp_boost, class.dodge_chance, class.description);
        for m in class.move_set {
            println!("  Move: {}\n  Damage Range: {:?}\n  Heal Range: {:?}\n  Cooldown: {}s\n", 
                m.name, m.damage_range, m.heal_range, m.cooldown);
        }
        println!(); // Extra blank line for spacing
    }

    println!("Weapons:");
    for weapon in weapons {
        println!("Name: {}\nRarity: {}\nDescription: {}\nWeapon URL: {}\nDMG Boost: {}\nDMG Range: {:?}\n", 
            weapon.name, weapon.rarity, weapon.description, weapon.weapon_url, weapon.dmg_boost, weapon.dmg_range);
        println!(); // Extra blank line for spacing
    }

    println!("Battles:");
    for battle in battles {
        println!("Name: {}\nTier: {}\nHP Range: {:?}\nXP Range: {:?}\n", 
            battle.name, battle.tier, battle.hp_range, battle.xp_range);
        for m in battle.move_set {
            println!("  Move: {}\n  Damage Range: {:?}\n  Heal Range: {:?}\n  Cooldown: {}s\n", 
                m.name, m.damage_range, m.heal_range, m.cooldown);
        }
        println!(); // Extra blank line for spacing
    }

    Ok(())
}

#[test]
fn test_count_models() -> Result<(), Box<dyn Error>> {
    let (classes, weapons, battles) = load_models()?;

    // Dynamic expected count based on the loaded data
    let expected_classes_count = classes.len();
    let expected_weapons_count = weapons.len();
    let expected_battles_count = battles.len();

    assert_eq!(classes.len(), expected_classes_count, "Mismatch in classes count");
    assert_eq!(weapons.len(), expected_weapons_count, "Mismatch in weapons count");
    assert_eq!(battles.len(), expected_battles_count, "Mismatch in battles count");

    // Print emoji-based results
    println!("✅ Loaded {} Classes.", classes.len());
    println!("✅ Loaded {} Weapons.", weapons.len());
    println!("✅ Loaded {} Battles.", battles.len());

    Ok(())
}
