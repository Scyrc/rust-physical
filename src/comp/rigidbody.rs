use bevy::math::Vec3;
use bevy::prelude::Component;

pub trait Simulate {
    fn step_velocity(&mut self, dt: f32, acceleration: Vec3);

    fn step_position(&mut self, dt: f32) -> Vec3;

    fn apply_impulse(&self, impulse: Vec3, r: Vec3);

    fn get_position(&mut self) -> Vec3;

}
#[derive(Component)]
pub struct RigidBody {
    position: Vec3,
    velocity: Vec3,
    angular_velocity: Vec3,
    angular_acceleration: Vec3,
    mass: f32,
    b_static: bool,
    b_sleep: bool,
    restitution: f32,
}

impl RigidBody {
    pub fn new(position: Vec3, velocity: Vec3, mass: f32, b_static: bool, b_sleep: bool) -> RigidBody {
        RigidBody{
            position,
            velocity,
            angular_velocity: Default::default(),
            angular_acceleration: Default::default(),
            mass,
            b_static,
            b_sleep,
            restitution: 0.0,
        }
    }
}

impl Default for RigidBody {
    fn default() -> RigidBody {
        RigidBody{
            position: Vec3::new(0.0, 5.0, 0.0),
            velocity: Vec3::new(0.0, 0.0, 0.0),
            angular_velocity: Default::default(),
            angular_acceleration: Default::default(),
            mass: 1.0,
            b_static: false,
            b_sleep: false,
            restitution: 0.0,
        }
    }
}

#[derive(Component)]
pub struct CuboidBody {
    rigid_body: RigidBody,
    x_size: f32,
    y_size: f32,
    z_size: f32,
}

impl Default for CuboidBody {
    fn default() -> CuboidBody {
        CuboidBody{
            rigid_body: RigidBody::default(),
            x_size: 1.0,
            y_size: 1.0,
            z_size : 1.0,
        }
    }
}

impl Simulate for CuboidBody {
    fn step_velocity(&mut self, dt: f32, acceleration: Vec3) {
        self.rigid_body.velocity += acceleration * dt;
    }

    fn step_position(&mut self, dt: f32) -> Vec3 {
        self.rigid_body.position += self.rigid_body.velocity * dt;
        self.rigid_body.position
    }


    fn apply_impulse(&self, impulse: Vec3, r: Vec3) {}

    fn get_position(&mut self) -> Vec3 {
        self.rigid_body.position
    }
}

