use crate::config::{HEIGHT, WIDTH};
use bevy::{prelude::*, window::*};

pub struct DisplayPlugin;

impl Plugin for DisplayPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
            .add_plugins((DefaultPlugins
                .set(window_plugin())
                .set(ImagePlugin::default_nearest()),))
            .add_systems(Startup, setup)
            .add_systems(Update, close_on_esc);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn window_plugin() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "Street Fighter II".into(),
            resolution: WindowResolution::new(WIDTH, HEIGHT).with_scale_factor_override(1.),
            // mode: WindowMode::Fullscreen,
            present_mode: PresentMode::AutoVsync,
            // Tells wasm to resize the window according to the available canvas
            fit_canvas_to_parent: true,
            // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
            prevent_default_event_handling: false,
            window_theme: Some(WindowTheme::Dark),
            position: WindowPosition::Centered(MonitorSelection::Primary),
            resizable: false,
            // enabled_buttons: EnabledButtons {
            //     maximize: false,
            //     ..default()
            // },
            // cursor: Cursor {
            //     visible: false,
            //     ..default()
            // },
            // visible: false,
            ..default()
        }),
        ..default()
    }
}

// fn scale_window(mut window: Query<&mut Window>) {
//     let mut window = window.single_mut();
//     let scale_factor = 4f32;
//     window
//         .resolution
//         .set_scale_factor_override(Some(scale_factor as f64));
//     let displacement_factor = (scale_factor - 1.) / 2.;
//     let x = WIDTH * displacement_factor;
//     let y = HEIGHT * displacement_factor;
//
//     let (initial_x, initial_y) = match window.position {
//         WindowPosition::At(v) => (v.x, v.y),
//         _ => (0, 0),
//     };
//
//     window
//         .position
//         .set(IVec2::new(initial_x - x as i32, initial_y - y as i32));
//
//     window.visible = true;
// }
