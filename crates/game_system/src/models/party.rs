use std::collections::HashMap;
use bevy::prelude::*;
use crate::characters::Character;

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

// ================================================================
//                               Tests
// ================================================================

#[cfg(test)]
mod tests {
    use super::*;

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