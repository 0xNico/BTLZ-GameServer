// src/read_models.rs
use crate::models::{Classes, Class, Weapons, Weapon,  Battles, Battle};
use std::error::Error;
use serde_json::from_str;

pub fn load_models() -> Result<(Vec<Class>, Vec<Weapon>, Vec<Battle>), Box<dyn Error>> {
    let classes_data = std::fs::read_to_string("models/classes.json")?;
    let weapons_data = std::fs::read_to_string("models/weapons.json")?;
    let battles_data = std::fs::read_to_string("models/battles.json")?;

    let classes: Classes = from_str(&classes_data)?;
    let weapons: Weapons = from_str(&weapons_data)?;
    let battles: Battles = from_str(&battles_data)?;

    Ok((classes.classes, weapons.weapons, battles.battles))
}
