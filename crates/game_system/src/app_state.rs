use bevy::prelude::*;

/// Represents the high-level game states used to control the flow of the application.
///
/// This enum is used with the Bevy `States` system to manage transitions between major phases
/// of the game, such as loading, splash screen, title menu, account handling, and gameplay.
///
/// It is also marked as a `Resource` to allow global access and manipulation via systems.
/// Each state represents a logical stage in the lifecycle of the game:
/// - Initialization (`Startup`)
/// - Logo screen (`SplashScreen`)
/// - Title and main menu (`TitleScreen`)
/// - User account input (`AccountScreen`)
/// - Server data fetch (`Preload`)
/// - Asset loading (`LoadGameAssets`)
/// - Actual gameplay (`InGame`)
///
/// Used with `NextState<GameState>` to transition between states at runtime.
#[derive(States, Resource, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Startup,
    #[default]
    SplashScreen,
    TitleScreen,
    AccountScreen,
    Preload,
    PreloadEnv,
    LoadGameAssets,
    PostLoad,
    InGame,
}