// tests/view_models_test.rs
use btlz_rust::view_models;
use std::error::Error;

#[test]
fn test_view_models() -> Result<(), Box<dyn Error>> {
    let (classes, weapons, battles) = view_models::view_models()?;

    // Assert there are exactly 5 of each
    assert_eq!(classes.len(), 5, "There should be exactly 5 classes");
    assert_eq!(weapons.len(), 5, "There should be exactly 5 weapons");
    assert_eq!(battles.len(), 5, "There should be exactly 5 battles");

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
