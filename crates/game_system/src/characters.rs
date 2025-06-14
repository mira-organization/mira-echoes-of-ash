use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::models::effects::Effects;

/// Represents a playable or AI-controlled character with all necessary
/// stats, attributes, model information, and combat-related data.
#[derive(Component, Reflect, Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Character {
    pub name: String,
    pub model_path: String,
    pub in_world_attack_range: f32,
    pub in_world: bool,
    pub skill_attributes: CharacterSkillAttributes,
    pub current_stats: CharacterCurrentStats,
    pub base_attributes: CharacterBaseAttributes,
    pub extra_attributes: CharacterExtraAttributes,
    pub damage_attributes: CharacterDamageAttributes,
    pub effects: Vec<Effects>,
}

/// Contains the character's current in-game stats,
/// such as health, attack, and speed, which may change during gameplay.
#[derive(Component, Reflect, Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CharacterCurrentStats {
    pub hp: f64,
    pub ability_points: f64,
    pub super_armor: f64,
    pub attack: f64,
    pub defense: f64,
    pub speed: f64,
    pub crit_rate: f64,
    pub crit_damage: f64,
}

/// Represents the character's base stats before any modifications,
/// typically used as the starting point or baseline values.
#[derive(Component, Reflect, Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CharacterBaseAttributes {
    pub hp: f64,
    pub ability_points: f64,
    pub super_armor: f64,
    pub attack: f64,
    pub defense: f64,
    pub speed: f64,
    pub crit_rate: f64,
    pub crit_damage: f64,
}

/// Holds additional passive or equipment-based bonuses that modify
/// the character's base stats or abilities.
#[derive(Component, Reflect, Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CharacterExtraAttributes {
    pub bonus_heal: f64,
}

/// Describes all elemental or magical damage types the character
/// can deal, including raw damage and "wds" modifiers for each type.
#[derive(Component, Reflect, Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CharacterDamageAttributes {
    pub fire_damage: f64,
    pub fire_wds: f64,
    pub water_damage: f64,
    pub water_wds: f64,
    pub earth_damage: f64,
    pub earth_wds: f64,
    pub air_damage: f64,
    pub air_wds: f64,
    pub lightning_damage: f64,
    pub lightning_wds: f64,
    pub dark_damage: f64,
    pub dark_wds: f64,
    pub holy_damage: f64,
    pub holy_wds: f64,
    pub ice_damage: f64,
    pub ice_wds: f64,
}

/// Contains the RPG-style attribute values that influence
/// derived stats, skill scaling, and other gameplay mechanics.
#[derive(Component, Reflect, Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CharacterSkillAttributes {
    pub vitality: f64,
    pub strength: f64,
    pub dexterity: f64,
    pub constitution: f64,
    pub intelligence: f64,
    pub luck: f64,
}

// ================================================================
//                               Tests
// ================================================================

#[cfg(test)]
mod unit_tests {
    use super::*;

    fn create_test_character() -> Character {
        Character {
            name: "Mira".to_string(),
            model_path: "models/mira.glb".to_string(),
            in_world_attack_range: 2.5,
            in_world: false,
            skill_attributes: CharacterSkillAttributes {
                vitality: 10.0,
                strength: 8.0,
                dexterity: 7.0,
                constitution: 9.0,
                intelligence: 6.0,
                luck: 5.0,
            },
            current_stats: CharacterCurrentStats {
                hp: 100.0,
                ability_points: 50.0,
                super_armor: 20.0,
                attack: 15.0,
                defense: 10.0,
                speed: 5.0,
                crit_rate: 0.1,
                crit_damage: 1.5,
            },
            base_attributes: CharacterBaseAttributes {
                hp: 100.0,
                ability_points: 50.0,
                super_armor: 20.0,
                attack: 15.0,
                defense: 10.0,
                speed: 5.0,
                crit_rate: 0.1,
                crit_damage: 1.5,
            },
            extra_attributes: CharacterExtraAttributes {
                bonus_heal: 5.0,
            },
            damage_attributes: CharacterDamageAttributes {
                fire_damage: 10.0,
                fire_wds: 1.2,
                water_damage: 0.0,
                water_wds: 1.0,
                earth_damage: 0.0,
                earth_wds: 1.0,
                air_damage: 0.0,
                air_wds: 1.0,
                lightning_damage: 0.0,
                lightning_wds: 1.0,
                dark_damage: 0.0,
                dark_wds: 1.0,
                holy_damage: 0.0,
                holy_wds: 1.0,
                ice_damage: 0.0,
                ice_wds: 1.0,
            },
            effects: vec![],
        }
    }

    #[test]
    fn test_character_creation() {
        let character = create_test_character();
        assert_eq!(character.name, "Mira");
        assert_eq!(character.current_stats.hp, 100.0);
        assert_eq!(character.skill_attributes.vitality, 10.0);
        assert!(character.effects.is_empty());
    }

    #[test]
    fn test_damage_attributes_fire() {
        let character = create_test_character();
        assert_eq!(character.damage_attributes.fire_damage, 10.0);
        assert!(character.damage_attributes.water_damage <= 0.0);
    }

    #[test]
    fn test_critical_stats() {
        let character = create_test_character();
        assert!(character.current_stats.crit_rate >= 0.0);
        assert!(character.current_stats.crit_damage > 1.0);
    }

    #[test]
    fn test_model_path_valid() {
        let character = create_test_character();
        assert!(character.model_path.ends_with(".glb"));
    }
}