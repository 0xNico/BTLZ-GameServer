// src/routes.rs
use actix_web::{web, HttpResponse, Responder};
use crate::read_models::load_models;
use crate::models::{Class, Weapon, Battle, ActiveBattles};
use crate::monster_utils::select_monster_for_battle;
use serde::Serialize;
use serde_json::json;
use log::{info, warn};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/view_models", web::get().to(view_models))
       .route("/count_models", web::get().to(count_models))
       .route("/models_json", web::get().to(models_json))
       .route("/create_battle", web::get().to(create_battle))
       .route("/join_battle", web::get().to(join_battle));
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

async fn create_battle(active_battles: web::Data<ActiveBattles>) -> impl Responder {
    let battle_instance = select_monster_for_battle();
    
    let mut battles = active_battles.battles.lock().unwrap();
    battles.insert(battle_instance.id.clone(), battle_instance.clone());

    info!("⚔️ - Battle created at id: {}", battle_instance.id);

    HttpResponse::Ok().json(&battle_instance)
}

async fn join_battle(active_battles: web::Data<ActiveBattles>) -> HttpResponse {
    let mut battles = active_battles.battles.lock().unwrap();

    if let Some((id, battle)) = battles.iter_mut().find(|(_, b)| !b.player_joined) {
        battle.player_joined = true;
        info!("🤺 - Player joined battle at id: {}", id);
        HttpResponse::Ok().json(json!({
            "message": "Successfully joined the battle.",
            "battle_id": id,
            "monster": battle.monster,
            "xp_to_give": battle.xp_to_give,
            "battle_hp": battle.battle_hp,
        }))
    } else {
        warn!("No available battles to join.");
        HttpResponse::NotFound().json(json!({"message": "No available battles to join."}))
    }
}