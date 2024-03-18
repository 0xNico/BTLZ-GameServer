// src/monster_utils.rs
use crate::models::{Battle, BattleInstance, Battles};
use rand::Rng;
use serde_json::from_str;
use std::fs;
use uuid::Uuid;

pub fn select_monster_for_battle() -> BattleInstance {
    let battles_data = fs::read_to_string("models/battles.json")
        .expect("Failed to read battles.json");

    // Assuming the structure of battles.json is a wrapper around an array
    let wrapper: Battles = from_str(&battles_data)
        .expect("Failed to parse battles.json");

    let mut rng = rand::thread_rng();
    let selected_battle = wrapper.battles.get(rng.gen_range(0..wrapper.battles.len()))
        .expect("Failed to select a random battle").clone();

    let hp_range = &selected_battle.hp_range;
    let xp_range = &selected_battle.xp_range;

    let battle_hp = rng.gen_range(hp_range[0]..=hp_range[1]);
    let xp_to_give = rng.gen_range(xp_range[0]..=xp_range[1]);

    BattleInstance {
        id: Uuid::new_v4().to_string(),
        monster: selected_battle,
        xp_to_give,
        battle_hp,
    }
}
