// src/models.rs - * classes, weapons and battles populate structs from API.
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Classes {
    pub classes: Vec<Class>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weapons {
    pub weapons: Vec<Weapon>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Battles {
    pub battles: Vec<Battle>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Class {
    pub name: String,
    pub armor_url: String,
    pub hp_boost: f64,
    pub dodge_chance: f64,
    pub description: String,
    pub move_set: Vec<Move>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Battle {
    pub name: String,
    pub tier: i32,
    pub hp_range: Vec<i32>,
    pub xp_range: Vec<i32>,
    pub move_set: Vec<Move>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weapon {
    pub name: String,
    pub rarity: String,
    pub description: String,
    pub weapon_url: String,
    pub dmg_boost: f64,
    pub dmg_range: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Move {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damage_range: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heal_range: Option<Vec<i32>>,
    pub cooldown: f64,
}
