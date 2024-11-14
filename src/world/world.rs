use bevy::math::vec3;
use bevy::prelude::*;

use crate::comp::rigidbody::{CuboidBody, Simulate};

pub struct WorldPlugin;


impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_world);
        app.add_systems(FixedUpdate, tick);
    }
}

fn init_world(mut commands: Commands,
              mut meshes: ResMut<Assets<Mesh>>,
              mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });


    // cube
    let mut cuboid_body = CuboidBody::default();
    let init_pos = cuboid_body.get_position();
    commands.spawn((PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::srgb_u8(124, 144, 255)),
        transform: Transform::from_xyz(init_pos.x, init_pos.y, init_pos.z),
        ..default()
    }, cuboid_body));
}

fn tick(
    time: Res<Time>,
    mut query: Query<(&mut CuboidBody, &mut Transform)>,
) {
    let gravity = vec3(0.0, -9.8, 0.0);
    let delta_time = time.delta_seconds();
    for (mut body, mut transform) in query.iter_mut() {
        body.step_velocity(delta_time, gravity);
        let new_pos = body.step_position(delta_time);
        transform.translation = new_pos;
    }
}