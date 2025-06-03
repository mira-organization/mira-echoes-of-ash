use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use bevy::image::ImageSamplerDescriptor;
use bevy::input::common_conditions::input_toggle_active;
use bevy::log::{BoxedLayer, Level, LogPlugin};
use bevy::prelude::*;
use bevy::render::render_resource::WgpuFeatures;
use bevy::render::RenderPlugin;
use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};
use bevy::window::WindowResolution;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use chrono::Utc;
use tracing_subscriber::fmt::writer::BoxMakeWriter;
use tracing_subscriber::Layer;

/// Default logging filter used by the application.
/// Controls verbosity for various crates and modules.
const LOG_ENV_FILTER: &str = "info,\
wgpu_core=warn,wgpu_hal=error,\
offset_allocator=error,\
bevy_gltf=error, \
system=debug,\
naga=warn,\
bevy_render=info,\
symphonia_core=warn,\
symphonia_format_ogg=warn,\
symphonia_codec_vorbis=warn,\
mira_moba=debug";

/// Configuration options for the client application.
#[derive(Debug)]
pub struct ClientOptions {
    pub window_title: String,
    pub window_width: f32,
    pub window_height: f32,
}

impl Default for ClientOptions {
    /// Returns the default client configuration.
    ///
    /// # Defaults
    /// - `window_title`: `"Game Title"`
    /// - `window_width`: `1280.0`
    /// - `window_height`: `720.0`
    fn default() -> Self {
        Self {
            window_title: String::from("Game Title"),
            window_width: 1280.0,
            window_height: 720.0,
        }
    }
}

/// Application entry point for debug builds.
/// Sets up the app with development-specific plugins like E-gui and WorldInspector.
#[cfg(debug_assertions)]
fn main() -> AppExit {
    let options = ClientOptions::default();
    let mut app = App::new();
    client_dev_core(&mut app, options).run()
}

/// Application entry point for release builds.
/// Runs the core client without additional debugging plugins.
#[cfg(not(debug_assertions))]
fn main() -> AppExit {
    let options = ClientOptions::default();
    let mut app = App::new();
    client_release_core(&mut app, options).run()
}

/// Initializes the application for development mode.
///
/// # Parameters
/// - `app`: A mutable reference to the [`App`] instance.
/// - `options`: [`ClientOptions`] containing window configuration.
///
/// # Returns
/// A mutable reference to the configured [`App`] instance.
#[allow(dead_code)]
fn client_dev_core(app: &mut App, options: ClientOptions) -> &mut App {
    init_bevy_app(app, options)
        .add_plugins(EguiPlugin { enable_multipass_for_primary_context: true })
        .add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F3)))
}

/// Initializes the application for release mode.
///
/// # Parameters
/// - `app`: A mutable reference to the [`App`] instance.
/// - `options`: [`ClientOptions`] containing window configuration.
///
/// # Returns
/// A mutable reference to the configured [`App`] instance.
#[allow(dead_code)]
fn client_release_core(app: &mut App, options: ClientOptions) -> &mut App {
    init_bevy_app(app, options)
}

/// Initializes core Bevy app plugins and logging settings.
///
/// # Parameters
/// - `app`: A mutable reference to the [`App`] instance.
/// - `options`: [`ClientOptions`] containing window configuration.
///
/// # Returns
/// A mutable reference to the initialized [`App`] instance.
fn init_bevy_app(app: &mut App, options: ClientOptions) -> &mut App {
    app.add_plugins(DefaultPlugins.set(
        WindowPlugin {
            primary_window: Some(Window {
                title: options.window_title,
                resolution: WindowResolution::new(options.window_width, options.window_height),
                ..default()
            }),
            ..default()
        }
    ).set(
        RenderPlugin {
            render_creation: RenderCreation::Automatic(create_gpu_settings()),
            ..default()
        }
    ).set(ImagePlugin {
        default_sampler: ImageSamplerDescriptor::nearest(),
        ..default()
    }).set(LogPlugin {
        level: Level::DEBUG,
        filter: LOG_ENV_FILTER.to_string(),
        custom_layer: log_file_appender
    }))
}

/// Creates GPU settings for rendering.
///
/// The settings include enabling Vulkan as the rendering backend and enabling
/// the `POLYGON_MODE_LINE` feature for wireframe rendering.
///
/// # Returns
/// A configured [`WgpuSettings`] instance.
///
/// # Example
/// ```rust
/// let gpu_settings = create_gpu_settings();
/// ```
fn create_gpu_settings() -> WgpuSettings {
    WgpuSettings {
        features: WgpuFeatures::POLYGON_MODE_LINE,
        backends: Some(Backends::PRIMARY),
        ..default()
    }
}

/// Initializes a log file appender for the application.
///
/// This function creates a `logs` directory if it does not exist and generates a log file
/// with a timestamped name in the format `bevy-DD-MM-YYYY.log`. It then sets up a
/// logging layer that writes log messages to this file.
///
/// # Parameters
/// - `_app`: A mutable reference to the Bevy `App`. (Currently unused)
///
/// # Returns
/// - `Some(BoxedLayer)`: If the log file was successfully created and opened.
/// - `None`: If there was an error creating the log directory or opening the file.
///
/// # Logging Details
/// - The log file is set up to append new logs.
/// - ANSI formatting is disabled for better readability in plain text files.
/// - The log writer ensures proper synchronization using `Arc<Mutex<File>>`.
fn log_file_appender(_app: &mut App) -> Option<BoxedLayer> {
    let log_dir = PathBuf::from("logs");
    if let Err(e) = std::fs::create_dir_all(&log_dir) {
        eprintln!("Failed to create log directory: {}", e);
        return None;
    }
    
    let timestamp = Utc::now().format("bevy-%d-%m-%Y.log").to_string();
    let log_path = log_dir.join(timestamp);
    
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
        .ok()?;
    
    let file_arc = Arc::new(Mutex::new(file));
    
    let _shutdown_logger = StartLogText {
        file: Arc::clone(&file_arc),
    };
    
    let writer = BoxMakeWriter::new(move || {
        let file = file_arc.lock().unwrap().try_clone().expect("Failed to clone log file handle");
        Box::new(file) as Box<dyn Write + Send>
    });
    
    Some(Box::new(tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_writer(writer)
        .boxed()
    ))
}

/// Helper struct to insert a start log entry when logging is initialized.
///
/// When this struct is dropped, it writes a separator message to the log file
/// to indicate when logging has started.
struct StartLogText {
    file: Arc<Mutex<File>>, // Shared reference to the log file
}

impl Drop for StartLogText {
    fn drop(&mut self) {
        let mut file = self.file.lock().unwrap();
        let _ = writeln!(
            file,
            "\n====================================== [ Start ] ======================================\n"
        );
        let _ = file.flush();
    }
}

// ================================================================
//                               Tests
// ================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Read;
    use tempfile::NamedTempFile;

    #[test]
    fn default_client_options_are_correct() {
        let opts = ClientOptions::default();
        assert_eq!(opts.window_title, "Game Title");
        assert_eq!(opts.window_width, 1280.0);
        assert_eq!(opts.window_height, 720.0);
    }

    #[test]
    fn log_env_filter_contains_important_filters() {
        assert!(LOG_ENV_FILTER.contains("wgpu_core=warn"));
        assert!(LOG_ENV_FILTER.contains("system=debug"));
        assert!(LOG_ENV_FILTER.contains("mira_moba=debug"));
    }

    #[test]
    fn log_file_appender_creates_log_file() {

        let _ = fs::remove_dir_all("logs");
        let mut dummy_app = App::new();

        let layer = log_file_appender(&mut dummy_app);

        assert!(layer.is_some());

        let entries = fs::read_dir("logs")
            .expect("Log directory should exist")
            .filter_map(Result::ok)
            .collect::<Vec<_>>();

        assert!(
            entries.iter().any(|entry| {
                entry
                    .file_name()
                    .to_string_lossy()
                    .starts_with("bevy-")
            }),
            "Expected a log file starting with 'bevy-'"
        );
    }

    #[test]
    fn start_log_text_writes_separator_on_drop() {
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let path = temp_file.path().to_path_buf();

        {
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&path)
                .expect("Failed to open temp file");

            let arc = Arc::new(Mutex::new(file));
            let _logger = StartLogText { file: Arc::clone(&arc) };
            // Drop happens here
        }

        let mut contents = String::new();
        File::open(&path)
            .expect("Failed to reopen temp file")
            .read_to_string(&mut contents)
            .expect("Failed to read log file");

        assert!(
            contents.contains("[ Start ]"),
            "Expected log to contain start separator"
        );
    }
}

