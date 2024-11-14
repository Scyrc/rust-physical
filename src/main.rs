use bevy::prelude::*;

mod scene;
mod world;
mod config;
mod comp;

use scene::camera::CameraPlugin;
use world::world::WorldPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, CameraPlugin, WorldPlugin));
    app.add_systems(Startup, setup);
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

}
