use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents a gameplay effect applied to a character or entity.
///
/// Effects typically influence stats or abilities temporarily,
/// and are defined by a name and duration. This struct is designed
/// to be serializable for saving/loading game state, and reflectable
/// for runtime inspection (e.g., in editor/debug tools).
#[derive(Component, Reflect, Debug, Clone, Default, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Effects {
    pub name: String,
    pub duration: f64,
}

// ================================================================
//                               Tests
// ================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_create_effect() {
        let effect = Effects {
            name: "Burning".to_string(),
            duration: 5.0,
        };

        assert_eq!(effect.name, "Burning");
        assert_eq!(effect.duration, 5.0);
    }

    #[test]
    fn test_serialize_effect() {
        let effect = Effects {
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
        let effect: Effects = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(effect.name, "Poisoned");
        assert_eq!(effect.duration, 7.5);
    }
}