use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use game_core::states::{AppState, BeforeUiState};


/// Marker component applied to the root UI node of the splash screen.
///
/// This allows systems and queries to identify and operate on the splash
/// screen’s root entity during setup and teardown.
#[derive(Component)]
struct SplashRoot;

/// Marker component for the full-screen overlay node used to perform
/// fade-in and fade-out transitions on the splash screen.
///
/// Systems will query for this component to adjust its background
/// color’s alpha channel over time.
#[derive(Component)]
struct FadeOverlay;

/// Resource that tracks the display duration of the initial “studio”
/// splash screen phase.
///
/// Internally wraps a `Timer` configured with a one-shot duration.
/// Systems tick this timer each frame and check when it finishes
/// to proceed to the next phase.
#[derive(Resource, Deref, DerefMut, Default)]
struct SplashTimer(Timer);

/// Resource that tracks the display duration of the “powered by”
/// splash screen phase.
///
/// Internally wraps a `Timer`. Like `SplashTimer`, it is reset and
/// ticked each frame to determine when to fade out the powered screen.
#[derive(Resource, Deref, DerefMut, Default)]
struct SplashPhaseTimer(Timer);

/// Resource-controlling fade transitions for the splash overlay.
///
/// Fields:
/// - `timer`: a one-shot `Timer` that drives each fade cycle.
/// - `from`: starting alpha value (0.0 = fully transparent, 1.0 = opaque).
/// - `to`: target alpha value for the fade.
///
/// Systems update `timer` each frame, then interpolate the overlay’s
/// alpha from `from` to `to` over the timer’s duration.
#[derive(Resource)]
struct FadeTimer {
    timer: Timer,
    from: f32,
    to: f32,
}

/// Represents the distinct phases of the splash screen’s fade state
/// machine.
///
/// Transitions between these states drive the sequence:
/// 1. `FadeInStudio` – fade in studio title
/// 2. `StudioDisplay` – hold the title on-screen
/// 3. `FadeOutStudio` – fade out the studio title
/// 4. `FadeInPowered` – fade in the “powered by” UI
/// 5. `PoweredDisplay` – hold the “powered by” UI on-screen
/// 6. `FadeOutPowered` – fade out and finish the splash sequence
#[derive(States, Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SplashFadePhase {
    /// Initial state: perform fade-in of the studio title overlay.
    #[default]
    FadeInStudio,
    /// After fade-in completing, display the studio title for a fixed duration.
    StudioDisplay,
    /// Fade out the studio title overlay to prepare for the next screen.
    FadeOutStudio,
    /// Fade in the “powered by” text and logo.
    FadeInPowered,
    /// After fade-in completes, display the “powered by” UI for a fixed duration.
    PoweredDisplay,
    /// Final fade-out of the powered UI, then cleanup and advance game state.
    FadeOutPowered,
}

pub struct SplashScreen;

impl Plugin for SplashScreen {

    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.init_state::<SplashFadePhase>();
        app.insert_resource(SplashTimer(Timer::from_seconds(2.0, TimerMode::Once)))
            .insert_resource(SplashPhaseTimer(Timer::from_seconds(2.0, TimerMode::Once)));
        app.insert_resource(FadeTimer {
            timer: Timer::from_seconds(1.0, TimerMode::Once),
            from: 1.0,
            to: 0.0,
        });
        app.add_systems(OnEnter(AppState::Screen(BeforeUiState::Splash)), create_studio_screen);
        app.add_systems(Update, (splash_screen_update, fade_overlay_system).run_if(in_state(AppState::Screen(BeforeUiState::Splash))));
    }
}

/// Spawns the initial splash screen UI, including the studio title
/// and a full-screen overlay for fade effects.
///
/// This system runs once when entering the `SplashScreen` state and:
/// 1. Creates a root `Node` covering the entire viewport with a dark
///    background and centered column layout.
/// 2. Adds a child `Text` entity displaying the studio name.
/// 3. Spawns a second `Node` tagged `FadeOverlay` on top of everything
///    (absolute positioning), which will be used to render fade-in/out
///    transitions by adjusting its background color’s alpha channel.
///
/// # Parameters
/// - `commands`: Used to spawn and configure UI entities.
///
#[coverage(off)]
fn create_studio_screen(mut commands: Commands) {
    commands.spawn((
        Name::new("Splashscreen"),
        SplashRoot,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::Srgba(Srgba::rgb_u8(20, 25,27))),
        ZIndex(1),
        RenderLayers::layer(1)
    ))
        .with_children(|ui| {
            ui.spawn((
                Name::new("Company"),
                Text::new("Tilt-Us Studio"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                RenderLayers::layer(1),
            ));
        });

    commands.spawn((
        FadeOverlay,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::Srgba(Srgba::rgb_u8(20, 25,27))),
        ZIndex(10),
        RenderLayers::layer(1),
    ));
}

/// Advances the splash screen state machine on each frame, handling:
/// - The studio display timer (allowing skip via space/escape/mouse)
/// - Transitioning between phases by resetting and configuring the
///   shared `FadeTimer`.
/// - Despawning and respawning UI elements when moving to the “Powered”
///   screen, including loading the Bevy logo asset.
/// - Final cleanup when fading out the powered screen and advancing
///   the global `AppState`.
///
/// # Parameters
/// - `commands`: To spawn/despawn UI entities.
/// - `time`: Provides delta time for advancing timers.
/// - `input` / `mouse`: To detect skip or proceed input events.
/// - `splash_timer`: Controls how long the studio screen stays visible.
/// - `phase_timer`: Controls how long the powered screen stays visible.
/// - `fade_timer`: Shared timer for fade-in/out transitions (opacity).
/// - `fade_phase`: Next state setter for `SplashFadePhase`.
/// - `current_fade_phase`: Current `SplashFadePhase` state guard.
/// - `next_game_state`: Sets the global `AppState` once splash completes.
/// - `root_query`: Identifies the root entity to despawn respawn children.
/// - `asset_server`: Used to load the Bevy logo image.
/// - `children`: Allows despawning old text before spawning powered UI.
/// - `fade_drop`: Identifies the overlay entity to despawn on exit.
///
#[coverage(off)]
fn splash_screen_update(
    mut commands: Commands,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut splash_timer: ResMut<SplashTimer>,
    mut phase_timer: ResMut<SplashPhaseTimer>,
    mut fade_timer: ResMut<FadeTimer>,
    mut fade_phase: ResMut<NextState<SplashFadePhase>>,
    current_fade_phase: Res<State<SplashFadePhase>>,
    mut next_game_state: ResMut<NextState<AppState>>,
    root_query: Query<Entity, With<SplashRoot>>,
    asset_server: Res<AssetServer>,
    mut children: Query<&mut Children>,
    fade_drop: Query<Entity, With<FadeOverlay>>
) {
    match current_fade_phase.get() {
        SplashFadePhase::FadeInStudio => {
            // handled by fade_overlay_system → transition to StudioDisplay
        }

        SplashFadePhase::StudioDisplay => {
            splash_timer.tick(time.delta());

            let proceed = splash_timer.finished()
                || input.any_pressed([KeyCode::Space, KeyCode::Escape])
                || mouse.any_pressed([MouseButton::Left, MouseButton::Right]);

            if proceed {
                fade_timer.timer.reset();
                fade_timer.from = 0.0;
                fade_timer.to = 1.0;
                fade_phase.set(SplashFadePhase::FadeOutStudio);
            }
        }

        SplashFadePhase::FadeOutStudio => {
            // handled by fade_overlay_system → transition to FadeInPowered
        }

        SplashFadePhase::FadeInPowered => {

            fade_timer.timer.reset();
            fade_timer.from = 1.0;
            fade_timer.to = 0.0;
            fade_phase.set(SplashFadePhase::PoweredDisplay);

            if let Ok(root) = root_query.single() {
                if let Ok(child_list) = children.get_mut(root) {
                    for child in child_list.iter() {
                        commands.entity(child).despawn();
                    }

                    // Add Powered By + Logo
                    commands.entity(root).with_children(|ui| {
                        ui.spawn((
                            Name::new("Bevy Text"),
                            Text::new("Powered By:"),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                            RenderLayers::layer(1),
                        ));

                        ui.spawn((
                            Name::new("Bevy Logo"),
                            ImageNode {
                                image: asset_server.load("images/bevy_logo_dark.png"),
                                ..default()
                            },
                            RenderLayers::layer(1),
                        ));
                    });
                }
            }
        }

        SplashFadePhase::PoweredDisplay => {
            phase_timer.tick(time.delta());

            let proceed = phase_timer.finished()
                || input.any_pressed([KeyCode::Space, KeyCode::Escape])
                || mouse.any_pressed([MouseButton::Left, MouseButton::Right]);

            if proceed {
                fade_timer.timer.reset();
                fade_timer.from = 0.0;
                fade_timer.to = 1.0;
                fade_phase.set(SplashFadePhase::FadeOutPowered);
            }
        }

        SplashFadePhase::FadeOutPowered => {
            if fade_timer
                .timer.finished() {
                if let Ok(root) = root_query.single() {
                    commands.entity(root).despawn();
                }

                if let Ok(fade_entity) = fade_drop.single() {
                    commands.entity(fade_entity).despawn();
                }

                next_game_state.set(AppState::Screen(BeforeUiState::Account));
            }
        }
    }
}

/// Drives the fade overlay’s opacity each frame, based on the active
/// `FadeTimer` and its `from`/`to` range, and triggers phase transitions
/// when the fade completes.
///
/// # Parameters
/// - `time`: Provides delta time for ticking the fade timer.
/// - `overlay_query`: Query for the `BackgroundColor` of the `FadeOverlay` node.
/// - `fade_timer`: Shared resource holding the fade timer and alpha range.
/// - `next_fade_phase`: Sets the upcoming `SplashFadePhase` when fade finishes.
/// - `current_fade_phase`: Guards which fade transition to apply next.
///
#[coverage(off)]
fn fade_overlay_system(
    time: Res<Time>,
    mut overlay_query: Query<&mut BackgroundColor, With<FadeOverlay>>,
    mut fade_timer: ResMut<FadeTimer>,
    mut next_fade_phase: ResMut<NextState<SplashFadePhase>>,
    current_fade_phase: Res<State<SplashFadePhase>>,
) {
    fade_timer.timer.tick(time.delta());

    let t = (fade_timer.timer.elapsed_secs() / fade_timer.timer.duration().as_secs_f32()).clamp(0.0, 1.0);
    let alpha = fade_timer.from + (fade_timer.to - fade_timer.from) * t;

    // Update the background color alpha based on the fade progress
    for mut color in &mut overlay_query {
        color.0.set_alpha(alpha);
    }

    // Handle phase transitions once the fade timer is finished
    if fade_timer.timer.finished() {
        match *current_fade_phase.get() {
            SplashFadePhase::FadeInStudio if t >= 1.0 => {
                next_fade_phase.set(SplashFadePhase::StudioDisplay);
            }
            SplashFadePhase::FadeOutStudio if t >= 1.0 => {
                next_fade_phase.set(SplashFadePhase::FadeInPowered);
            }
            SplashFadePhase::FadeInPowered if t >= 1.0 => {
                next_fade_phase.set(SplashFadePhase::PoweredDisplay);
            }
            SplashFadePhase::FadeOutPowered if t >= 1.0 => {
                // Transition will be handled in splash_screen_update
            }
            _ => {}
        }
    }
}
