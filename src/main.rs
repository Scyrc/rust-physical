use bevy::{
    core::FrameCount,
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    window::{PresentMode, WindowTheme},
};

mod scene;
mod world;
mod comp;
mod ui;

use crate::ui::event::EventPlugin;
use crate::ui::ui::UIPlugin;

use scene::camera::CameraControlPlugin;
use world::world::WorldPlugin;
fn main() {
    let mut app = App::new();
    let win_size = 1500.0;
    app.add_plugins((DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "rust simulation!".into(),
            name: Some("bevy.app".into()),
            resolution: (win_size, win_size * 0.618).into(),
            present_mode: PresentMode::AutoVsync,
            // Tells wasm to resize the window according to the available canvas
            fit_canvas_to_parent: true,
            // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
            prevent_default_event_handling: false,
            window_theme: Some(WindowTheme::Dark),
            enabled_buttons: bevy::window::EnabledButtons {
                maximize: false,
                ..Default::default()
            },
            // This will spawn an invisible window
            // The window will be made visible in the make_visible() system after 3 frames.
            // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
            visible: false,
            ..default()
        }),
        ..default()
    }),
                     FrameTimeDiagnosticsPlugin, EventPlugin, CameraControlPlugin, WorldPlugin , UIPlugin))

    .add_systems(Startup, setup)
        .add_systems(
            Update,
            (

                make_visible,
            ),
        );
    app.run();
}

fn setup(

) {



}

fn make_visible(mut window: Query<&mut Window>, frames: Res<FrameCount>) {
    // The delay may be different for your app or system.
    if frames.0 == 3 {
        // At this point the gpu is ready to show the app so we can make the window visible.
        // Alternatively, you could toggle the visibility in Startup.
        // It will work, but it will have one white frame before it starts rendering
        window.single_mut().visible = true;
    }
}
