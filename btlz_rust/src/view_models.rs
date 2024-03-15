// src/view_models.rs
use std::fs::File;
use std::io::prelude::*;
use std::io::Result as IoResult;
use std::error::Error;
use serde_json::from_str;
use crate::models::{Classes, Class, Weapons, Weapon, Battles, Battle};

fn read_file_to_string(file_path: &str) -> IoResult<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn view_models() -> Result<(Vec<Class>, Vec<Weapon>, Vec<Battle>), Box<dyn Error>> {
    let classes_data = read_file_to_string("models/classes.json")?;
    let weapons_data = read_file_to_string("models/weapons.json")?;
    let battles_data = read_file_to_string("models/battles.json")?;

    let classes_wrapper: Classes = from_str(&classes_data)?;
    let weapons_wrapper: Weapons = from_str(&weapons_data)?;
    let battles_wrapper: Battles = from_str(&battles_data)?;

    // Extract the inner vectors from the wrapper structs
    let classes = classes_wrapper.classes;
    let weapons = weapons_wrapper.weapons;
    let battles = battles_wrapper.battles;

    Ok((classes, weapons, battles))
}
