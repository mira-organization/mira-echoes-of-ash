#![coverage(off)]

use bevy::prelude::*;

/// Trait representing UI-relevant data for dialog-capable entities.
///
/// <p>Types implementing this trait (e.g., NPCs or world items)
/// provide dialog-related information, such as UI element identifiers
/// and the corresponding display text.</p>
pub trait DialogData {
    /// Returns the static CSS ID of the main dialog container for this data.
    ///
    /// <p>This is used to toggle visibility based on the dialog context.</p>
    fn dialog_visible_id(&self) -> &'static str;

    /// Returns a static placeholder ID for the dialog container.
    ///
    /// <p>This is used in generic contexts (e.g., when `data_opt` is `None`)
    /// to still retrieve a valid CSS ID without referencing actual data.</p>
    fn dialog_visible_id_placeholder() -> &'static str;

    /// Returns the static CSS ID for the title field of the dialog.
    fn title_id(&self) -> &'static str;

    /// Returns the static CSS ID for the text/paragraph field of the dialog.
    fn text_id(&self) -> &'static str;

    /// Optionally returns the static CSS ID for an icon/image field.
    ///
    /// <p>Defaults to `None` if the dialog does not include an icon.</p>
    fn icon_id(&self) -> Option<&'static str> {
        None
    }

    /// Returns the title text that should be displayed in the dialog UI.
    fn title_text(&self) -> String;

    /// Returns the main text of the dialog, including any interpolated keys.
    ///
    /// @param interact_key The keybinding or action used for interaction.
    fn main_text(&self, interact_key: &str) -> String;

    /// Optionally returns the icon image source path to display.
    ///
    /// <p>Defaults to `None` if no icon is used in the dialog.</p>
    fn icon(&self) -> Option<String> {
        None
    }
}

