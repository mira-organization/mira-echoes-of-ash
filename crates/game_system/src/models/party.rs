use std::collections::HashMap;
use bevy::prelude::*;
use crate::characters::Character;

#[derive(Resource, Default, Debug, Clone)]
pub struct CharacterPartyInfo(pub HashMap<String, Character>);

impl CharacterPartyInfo {
    pub fn add(&mut self, name: String, character: Character) {
        if self.0.get(&name).is_some() {
            debug!("{} already exists in your party!", name);
            return;
        }

        self.0.insert(name, character);
    }

    pub fn remove(&mut self, name: String) {
        if self.0.get(&name).is_none() {
            debug!("{} doesn't exists in your party!", name);
            return;
        }

        self.0.remove(&name);
    }

    pub fn get_as_vec(&self) -> Vec<&Character> {
        let mut vec = Vec::new();
        for (_, character) in self.0.iter() {
            vec.push(character);
        }
        vec
    }
}