use actix_web::{web, HttpResponse};
use crate::read_models::load_models;
use crate::models::{Class, Weapon, Battle};
use crate::monster_utils::select_monster_for_battle;
use serde::Serialize;
use serde_json::json;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/view_models", web::get().to(view_models))
       .route("/count_models", web::get().to(count_models))
       .route("/models_json", web::get().to(models_json))
       .route("/create_battle", web::get().to(create_battle));
}

async fn view_models() -> HttpResponse {
    match load_models() {
        Ok((classes, weapons, battles)) => {
            let mut response = String::new();

            response.push_str("Classes:\n");
            for class in classes {
                response.push_str(&format!("Name: {}\nArmor URL: {}\nHP Boost: {}\nDodge Chance: {}\nDescription: {}\n", 
                    class.name, class.armor_url, class.hp_boost, class.dodge_chance, class.description));
                for m in class.move_set {
                    response.push_str(&format!("  Move: {}\n  Damage Range: {:?}\n  Heal Range: {:?}\n  Cooldown: {}s\n", 
                        m.name, m.damage_range, m.heal_range, m.cooldown));
                }
                response.push_str("\n"); // Extra blank line for spacing
            }

            response.push_str("Weapons:\n");
            for weapon in weapons {
                response.push_str(&format!("Name: {}\nRarity: {}\nDescription: {}\nWeapon URL: {}\nDMG Boost: {}\nDMG Range: {:?}\n", 
                    weapon.name, weapon.rarity, weapon.description, weapon.weapon_url, weapon.dmg_boost, weapon.dmg_range));
                response.push_str("\n"); // Extra blank line for spacing
            }

            response.push_str("Battles:\n");
            for battle in battles {
                response.push_str(&format!("Name: {}\nTier: {}\nHP Range: {:?}\nXP Range: {:?}\n", 
                    battle.name, battle.tier, battle.hp_range, battle.xp_range));
                for m in battle.move_set {
                    response.push_str(&format!("  Move: {}\n  Damage Range: {:?}\n  Heal Range: {:?}\n  Cooldown: {}s\n", 
                        m.name, m.damage_range, m.heal_range, m.cooldown));
                }
                response.push_str("\n"); // Extra blank line for spacing
            }

            HttpResponse::Ok().content_type("text/plain").body(response)
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn count_models() -> HttpResponse {
    match load_models() {
        Ok((classes, weapons, battles)) => {
            let response = format!("Classes: {}\nWeapons: {}\nBattles: {}", classes.len(), weapons.len(), battles.len());
            HttpResponse::Ok().content_type("text/plain").body(response)
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn models_json() -> HttpResponse {
    match load_models() {
        Ok((classes, weapons, battles)) => {
            // Construct a struct that derives Serialize to easily convert our data to JSON
            #[derive(Serialize)]
            struct GameModels {
                classes: Vec<Class>,
                weapons: Vec<Weapon>,
                battles: Vec<Battle>,
            }

            let models = GameModels {
                classes,
                weapons,
                battles,
            };

            // Use HttpResponse::Ok().json() to automatically serialize and return JSON
            HttpResponse::Ok().json(models)
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn create_battle() -> HttpResponse {
    let battle_instance = select_monster_for_battle();

    HttpResponse::Ok().json(json!({
        "message": "Battle instance created",
        "battle_instance": {
            "id": battle_instance.id,
            "monster_name": battle_instance.monster.name,
            "xp_to_give": battle_instance.xp_to_give,
            "battle_hp": battle_instance.battle_hp,
        }
    }))
}