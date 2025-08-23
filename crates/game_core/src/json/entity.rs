use std::fs;
use std::path::Path;
use bevy::prelude::*;
use serde::Deserialize;
use crate::ENTITY_JSON_PATH;

/// Represents a game entity as defined in JSON data.
///
/// This struct is deserialized directly from JSON and contains all
/// information required to instantiate or reference an in‑game object,
/// including its display name, asset paths, available animations, and
/// category/type.
///
/// # Fields
///
/// - `localized_name`
///   A unique, internal identifier for the entity. This is typically used
///   in code or data references (e.g. `"characters::lira"`).
///
/// - `name`
///   A human‑friendly name for the entity, localized to the player's
///   language (e.g. `"Lira"`).
///
/// - `model_path`
///   Filesystem or asset‑bundle path pointing to the 3D model used to
///   render this entity. In the JSON source it is named `"model"`.
///
/// - `animations`
///   A list of `EntityAnimation` entries describing the animation clips
///   (e.g., walk, attack, idle) that this entity supports.
///
/// - `_type`
///   The classification of this entity (e.g., Character, Npc, Enemy,
///   Unknown). The JSON field is named `"type"`, but renamed here to
///   `_type` to avoid Rust keyword conflicts. If the JSON omits the
///   `"type"` field, it defaults to `EntityDataType::Unknown`.
///
#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
pub struct JsonEntity {
    pub localized_name: String,
    pub name: String,
    #[serde(rename = "model")]
    pub model_path: String,
    pub animations: Vec<EntityAnimation>,
    #[serde(default, rename = "type")]
    pub _type: EntityDataType
}

#[coverage(off)]
impl JsonEntity {

    /// Attempts to load and deserialize a single JSON‐defined entity by its
    /// “localized name” identifier and set its `EntityDataType` based on
    /// the folder it came from.
    ///
    /// The `localized_name` must be in the form `"folder::name"`, where:
    /// - `folder` is one of `"characters"`, `"enemies"`, `"npcs"`, or any other
    ///   folder name (unknown folders map to `EntityDataType::Unknown`)
    /// - `name` is the JSON file name without or with the `.json` extension
    ///
    /// This function:
    /// 1. Splits `localized_name` on `"::"` into `folder` and `name`.
    /// 2. Ensures `name` ends with `.json`, appending the suffix if necessary.
    /// 3. Builds a path `ENTITY_JSON_PATH/<folder>/data/<name>.json`.
    /// 4. Reads the file at that path; on I/O error, returns an `Err` with details.
    /// 5. Parses the file’s contents into `Self` via `serde_json::from_str`.
    ///    On parse failure, returns an `Err` describing the JSON error and path.
    /// 6. Overrides the entity’s `_type` field according to the `folder`:
    ///    - `"characters"` → `EntityDataType::Character`
    ///    - `"enemies"` → `EntityDataType::Enemy`
    ///    - `"npcs"` → `EntityDataType::Npc`
    ///    - anything else → `EntityDataType::Unknown`
    ///
    /// # Parameters
    ///
    /// - `localized_name`: A string slice in the format `"folder::name"`
    ///
    /// # Returns
    ///
    /// - `Ok(entity)` on success, where `entity` is the deserialized struct with
    ///   its `_type` field set appropriately.
    /// - `Err(String)` if:
    ///   - `localized_name` has no `folder` or `name` component
    ///   - the file cannot be read
    ///   - the JSON fails to parse
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Load "characters::hero" from
    /// // ENTITY_JSON_PATH/characters/data/hero.json
    /// let hero: JsonEntity = JsonEntity::fetch("characters::hero")?;
    /// assert_eq!(hero._type, EntityDataType::Character);
    /// ```
    #[coverage(off)]
    pub fn fetch(localized_name: &str) -> Result<Self, String>
    where
        Self: Sized + for<'de> Deserialize<'de>,
    {
        let mut parts = localized_name.split("::");
        let folder = parts.next().ok_or("No folder found in localized name.")?;
        let name = parts.next().ok_or("No name found in localized name.")?;

        let file_name = if name.ends_with(".json") {
            name.to_string()
        } else {
            format!("{}.json", name)
        };

        let path = Path::new(ENTITY_JSON_PATH)
            .join(folder)
            .join("data")
            .join(file_name);

        let file_content = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read file: {} : {}", path.display(), e))?;

        let mut entity: Self = serde_json::from_str(&file_content)
            .map_err(|e| format!("Failed to parse JSON in {:?}: {}", path, e))?;

        entity._type = match folder {
            "characters" => EntityDataType::Character,
            "enemies" => EntityDataType::Enemy,
            "npcs" => EntityDataType::Npc,
            _ => EntityDataType::Unknown,
        };

        Ok(entity)
    }

    /// Loads and deserializes all JSON‐defined entities found under the
    /// configured `ENTITY_JSON_PATH` directory.
    ///
    /// This function scans each subdirectory in `ENTITY_JSON_PATH`, expects a
    /// `data` folder within, and for each `.json` file found:
    /// 1. Constructs a localized identifier in the form `"folder::file_stem"`.
    /// 2. Calls `Self::fetch(&localized_name)` to load and parse the entity.
    /// 3. Pushes the resulting entity into the returned vector.
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<Self>)` containing one deserialized entity per JSON file.
    /// - `Err(String)` if any I/O or parsing step fails. Error messages include
    ///   context such as directory read failures, missing folder/file names,
    ///   or JSON parse errors.
    ///
    /// # Errors
    ///
    /// Possible failure points include:
    /// - Reading `ENTITY_JSON_PATH` itself or its subdirectories.
    /// - Converting directory entries or file stems to valid UTF‑8 strings.
    /// - Reading individual JSON files.
    /// - Parsing JSON contents via `serde_json`.
    /// - Any error from `Self::fetch`, which is propagated with additional context.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Assuming ENTITY_JSON_PATH has subfolders "characters", "enemies", "npcs"
    /// // each with a `data` directory full of JSON files:
    /// let all_entities: Vec<JsonEntity> = JsonEntity::fetch_all()?;
    /// println!("Loaded {} entities in total", all_entities.len());
    /// ```
    #[coverage(off)]
    pub fn fetch_all() -> Result<Vec<Self>, String>
    where
        Self: Sized + for<'de> serde::Deserialize<'de>,
    {
        let mut result = Vec::new();

        let dirs = fs::read_dir(ENTITY_JSON_PATH)
            .map_err(|e| format!("Failed to read directory: {}", e))?;

        for entry in dirs {
            let entry = entry.map_err(|e| format!("Failed to read dir entry: {}", e))?;
            let path = entry.path();

            if path.is_dir() {
                let parent_folder = path.file_name()
                    .and_then(|n| n.to_str())
                    .ok_or_else(|| format!("Failed to get folder name for {:?}", path))?;

                let data_path = path.join("data");
                if data_path.exists() && data_path.is_dir() {
                    for json_entry in fs::read_dir(&data_path)
                        .map_err(|e| format!("Failed to read data dir: {}", e))?
                    {
                        let json_entry = json_entry.map_err(|e| format!("Failed to read file: {}", e))?;
                        let json_path = json_entry.path();

                        if json_path.is_file()
                            && json_path.extension().and_then(|s| s.to_str()) == Some("json")
                        {
                            let stem = json_path.file_stem()
                                .and_then(|s| s.to_str())
                                .ok_or_else(|| format!("Failed to get file stem for {:?}", json_path))?;

                            let localized_name = format!("{}::{}", parent_folder, stem);

                            let entity = Self::fetch(&localized_name)
                                .map_err(|e| format!("Failed to fetch {}: {}", localized_name, e))?;
                            result.push(entity);
                        }
                    }
                }
            }
        }
        Ok(result)
    }

    /// Retrieves an animation by its name.
    ///
    /// # Arguments
    /// - `name` - The key of the animation.
    ///
    /// # Returns
    /// - `Some(&CharacterAnimation)` if found.
    /// - `None` if no matching animation exists.
    #[coverage(off)]
    pub fn get_animation_by_name(&self, name: &str) -> Option<&EntityAnimation> {
        self.animations.iter().find(|anim| anim.key == name)
    }
}

/// Describes a single animation clip for a game entity.
///
/// This struct is deserialized from JSON and associates a unique key
/// with an index into the entity’s animation data (e.g., a specific
/// frame or clip ID).
///
/// # Fields
///
/// - `key`: A string identifier for the animation (e.g. `"walk"`, `"attack"`).
/// - `index`: A numeric index pointing to the animation in the entity’s
///   animation list or asset bundle.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct EntityAnimation {
    pub key: String,
    pub index: u32,
}

/// The category of a JSON‑defined entity.
///
/// This enum is used to classify entities into one of several known types:
/// - `Character`
/// - `Npc`
/// - `Enemy`
/// - `Unknown` (default)
///
/// Defaults to `Unknown` if the JSON does not specify a type.
#[derive(Deserialize, Debug, Default, Clone, PartialEq)]
pub enum EntityDataType {
    Character,
    Npc,
    Enemy,
    #[default]
    Unknown,
}

impl EntityDataType {

    /// Returns the base asset path segment for this entity type’s model.
    ///
    /// This is appended to a root JSON directory to locate the
    /// corresponding `model.json` or asset folder.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// assert_eq!(
    ///     EntityDataType::Character.path(),
    ///     "/characters/model"
    /// );
    /// assert_eq!(
    ///     EntityDataType::Unknown.path(),
    ///     "/unknown"
    /// );
    /// ```
    #[coverage(off)]
    pub fn path(&self) -> &'static str {
        match self {
            EntityDataType::Character => "characters/model",
            EntityDataType::Npc => "npcs/model",
            EntityDataType::Enemy => "enemies/model",
            EntityDataType::Unknown => "unknown",
        }
    }
}