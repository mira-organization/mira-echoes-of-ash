#![coverage(off)]

use std::collections::HashMap;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::json::entity::JsonEntity;

/// Represents a playable or AI-controlled character with all necessary
/// stats, attributes, model information, and combat-related data.
#[derive(Component, Reflect, Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Character {
    pub name: String,
    #[serde(default)]
    pub model_path: String,
    #[serde(default)]
    pub in_world_attack_range: f32,
    #[serde(default)]
    pub in_world: bool,
    #[serde(default)]
    pub skill_attributes: CharacterSkillAttributes,
    #[serde(default)]
    pub current_stats: CharacterCurrentStats,
    #[serde(default)]
    pub base_attributes: CharacterBaseAttributes,
    #[serde(default)]
    pub extra_attributes: CharacterExtraAttributes,
    #[serde(default)]
    pub damage_attributes: CharacterDamageAttributes,
    #[serde(default)]
    pub effects: Vec<DotEffects>,
}

impl Character {

    #[coverage(off)]
    pub fn merge_json_character(&mut self, json_character: &JsonEntity) {
        self.model_path = json_character.model_path.clone();
    }

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

/// Represents a gameplay effect applied to a character or entity.
///
/// Effects typically influence stats or abilities temporarily,
/// and are defined by a name and duration. This struct is designed
/// to be serializable for saving/loading game state, and reflectable
/// for runtime inspection (e.g., in editor/debug tools).
#[derive(Component, Reflect, Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct DotEffects {
    pub name: String,
    pub duration: f64,
}

/// A resource that stores the current party members as a mapping of character names to `Character` instances.
///
/// This struct is used to manage the player's active party in the game world. It allows for adding,
/// removing, and querying characters by name, and is implemented as a wrapper around a `HashMap`.
///
/// The key is the character's name (`String`), and the value is the corresponding `Character` struct.
///
/// Example use cases include:
/// - Tracking which characters are currently in the player's party.
/// - Efficient lookups and updates by character name.
/// - Synchronizing party data with the save/load system or UI.
///
/// Can be registered as a `Resource` in the Bevy ECS for global access.
#[derive(Resource, Default, Debug, Clone)]
pub struct CharacterPartyInfo {
    pub members: HashMap<String, (usize, Character)>,
    pub active: Character
}


impl CharacterPartyInfo {
    /// Adds a character to the party by name.
    ///
    /// If a character with the given name already exists in the party, the function logs a message and does nothing.
    pub fn add(&mut self, name: String, character: Character) {
        if self.members.get(&name).is_some() {
            debug!("{} already exists in your party!", name);
            return;
        }

        let slot = self.members.len() + 1;
        self.members.insert(name, (slot, character));
    }

    /// Removes a character from the party by name.
    ///
    /// If no character with the given name exists in the party, the function logs a message and does nothing.
    pub fn remove(&mut self, name: String) {
        if self.members.get(&name).is_none() {
            debug!("{} doesn't exist in your party!", name);
            return;
        }

        self.members.remove(&name);
    }

    /// Returns the characters in the party as a vector of references.
    ///
    /// The order of characters in the vector is not guaranteed.
    pub fn get_as_vec(&self) -> Vec<(&usize, &Character)> {
        let mut vec = Vec::new();
        for (_, (slot, character)) in self.members.iter() {
            vec.push((slot, character));
        }
        vec
    }
}

/// A resource indicating whether the player wants to switch characters.
#[derive(Resource, Default, Clone, Debug)]
pub struct ChangeCharacter(pub bool);

/// A resource storing the currently active world character.
#[derive(Resource, Default, Clone, Debug)]
pub struct CurrentWorldCharacter(pub Option<(Entity, Character)>);

#[derive(Resource, Default, Deref, DerefMut)]
pub struct SlowWalkToggle(pub bool);

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_create_effect() {
        let effect = DotEffects {
            name: "Burning".to_string(),
            duration: 5.0,
        };

        assert_eq!(effect.name, "Burning");
        assert_eq!(effect.duration, 5.0);
    }

    #[test]
    fn test_serialize_effect() {
        let effect = DotEffects {
            name: "Frozen".to_string(),
            duration: 10.0,
        };

        let json = serde_json::to_string(&effect).expect("Failed to serialize");
        assert!(json.contains("Frozen"));
        assert!(json.contains("10.0"));
    }

    #[test]
    fn test_deserialize_effect() {
        let json = r#"{ "name": "Poisoned", "duration": 7.5 }"#;
        let effect: DotEffects = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(effect.name, "Poisoned");
        assert_eq!(effect.duration, 7.5);
    }

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

    fn dummy_character(name: &str) -> Character {
        Character {
            name: name.to_string(),
            model_path: "models/char.glb".to_string(),
            in_world_attack_range: 1.0,
            in_world: false,
            skill_attributes: Default::default(),
            current_stats: Default::default(),
            base_attributes: Default::default(),
            extra_attributes: Default::default(),
            damage_attributes: Default::default(),
            effects: vec![],
        }
    }

    #[test]
    fn test_add_character() {
        let mut party = CharacterPartyInfo::default();
        let character = dummy_character("Mira");

        party.add("Mira".to_string(), character.clone());

        assert_eq!(party.members.len(), 1);
        assert!(party.members.contains_key("Mira"));
    }

    #[test]
    fn test_add_duplicate_character() {
        let mut party = CharacterPartyInfo::default();
        let char1 = dummy_character("Liora");
        let char2 = dummy_character("Liora");

        party.add("Liora".to_string(), char1);
        party.add("Liora".to_string(), char2);

        assert_eq!(party.members.len(), 1);
    }

    #[test]
    fn test_remove_character() {
        let mut party = CharacterPartyInfo::default();
        let character = dummy_character("Ignara");

        party.add("Ignara".to_string(), character);
        party.remove("Ignara".to_string());

        assert!(!party.members.contains_key("Ignara"));
        assert_eq!(party.members.len(), 0);
    }

    #[test]
    fn test_remove_nonexistent_character() {
        let mut party = CharacterPartyInfo::default();

        party.remove("Unknown".to_string()); // Should do nothing

        assert_eq!(party.members.len(), 0);
    }

    #[test]
    fn test_get_as_vec() {
        let mut party = CharacterPartyInfo::default();
        party.add("Mira".to_string(), dummy_character("Mira"));
        party.add("Liora".to_string(), dummy_character("Liora"));

        let vec = party.get_as_vec();
        assert_eq!(vec.len(), 2);
        assert!(vec.iter().any(|(_, c)| c.name == "Mira"));
        assert!(vec.iter().any(|(_, c)| c.name == "Liora"));
    }
}