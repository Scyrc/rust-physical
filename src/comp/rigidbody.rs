use bevy::math::{vec3, EulerRot, Mat3, Quat, Vec3};
use bevy::prelude::{Component, Mut};
use std::ops::{Mul, Neg};

pub trait Simulate {
    fn step(&mut self, dt: f32, acceleration: Vec3);

    fn update_vel(&mut self, dt: f32);

    fn _apply_correction(&mut self, corr:Vec3, pos: Vec3);

    fn apply_correction(&mut self, compliance: f32, corr:Vec3, pos: Vec3, other_body:  Option<Mut<CuboidBody>>,other_pos: Vec3,)->f32;

    fn get_position(&mut self) -> Vec3;
    fn get_velocity(&self) -> Vec3;

    fn get_quat(&mut self) -> Quat;


    fn get_angule_vel(&self) -> Vec3;

    fn  get_mass(&self) -> f32;
    fn  get_mass_inv(&self, normal: Vec3, pos: Vec3) -> f32;


    fn local_to_world(&self, local_pos: Vec3) -> Vec3;

    fn world_to_local(&self, world_pos: Vec3) -> Vec3;


    fn get_inv_inertia(&self) -> Vec3;

    fn get_inertia(&self) -> Mat3;

    fn get_centroid_world_pos(&self) -> Vec3;
}

#[derive(Component)]
pub struct RigidBody {
    position: Vec3,
    velocity: Vec3,
    prev_pos: Vec3,
    angular_velocity: Vec3,
    mass: f32,
    b_static: bool,
    b_sleep: bool,
    //restitution: f32,
    quaternion: Quat,
    prev_quaternion: Quat,
    inv_quaternion: Quat,
    centroid: Vec3,
    inertia: Mat3,
    inv_mass: f32,
    inv_inertia: Vec3,
    density:f32,
    damping:f32,
    dt: f32,
}
impl RigidBody {
    fn new(pos:Vec3, angels: Vec3, density: f32) -> RigidBody {
        let rigid_body =  RigidBody{
            position: pos,
            velocity: Vec3::new(0.0, 0.0, 0.0),
            prev_pos: pos,
            angular_velocity: Default::default(),
            inv_mass: 0.0,
            mass: 1.0,
            b_static: false,
            b_sleep: false,
            quaternion: Quat::from_euler(EulerRot::XYZ,angels.x, angels.y, angels.z),
            prev_quaternion: Quat::from_euler(EulerRot::XYZ,angels.x, angels.y, angels.z),
            inv_quaternion: Quat::from_euler(EulerRot::XYZ, angels.x, angels.y, angels.z).inverse(),
            centroid: Vec3::ZERO,
            inertia: Mat3::IDENTITY,
            density,
            inv_inertia: Vec3::ONE,
            damping: 0.05,
            dt: 0.03,
        };
        rigid_body
    }

    fn init(&mut self, mass: f32, inv_mass: f32, inv_inertia: Vec3) {
        self.mass = mass;
        self.inv_mass = inv_mass;
        self.inv_inertia = inv_inertia;
    }
}

#[derive(Component)]
pub struct CuboidBody {
    rigid_body: RigidBody,
    x_size: f32,
    y_size: f32,
    z_size: f32,
    vertices: Vec<Vec3>,
    pub sleep_time:f32,
}

impl CuboidBody {
    pub fn new(pos:Vec3, size: Vec3, angles:Vec3, density: f32,) -> CuboidBody {
         let mut cuboid_body = CuboidBody{
            rigid_body: RigidBody::new(pos, angles, density),
            x_size: size.x,
            y_size: size.y,
            z_size : size.z,
            vertices: vec![],
            sleep_time: 0.0,
         };
        let mass = density * size.x * size.y * size.z;
        let inv_mass = 1.0 / mass;
        let ix = 1.0 / 12.0 * mass * (size.y * size.y + size.z * size.z);
        let iy = 1.0 / 12.0 * mass * (size.x * size.x + size.z * size.z);
        let iz = 1.0 / 12.0 * mass * (size.x * size.x + size.y * size.y);
        let inv_inertia =  Vec3::new(1.0 / ix, 1.0 / iy, 1.0 / iz);
        cuboid_body.rigid_body.init(mass, inv_mass, inv_inertia);
        let ex = 0.5 * size.x;
        let ey = 0.5 * size.y;
        let ez = 0.5 * size.z;

        cuboid_body.vertices.push(vec3(-ex, -ey, -ez));
        cuboid_body.vertices.push(vec3(ex, -ey, -ez));
        cuboid_body.vertices.push(vec3(ex, ey, -ez));
        cuboid_body.vertices.push(vec3(-ex, ey, -ez));
        cuboid_body.vertices.push(vec3(-ex, -ey, ez));
        cuboid_body.vertices.push(vec3(ex, -ey, ez));
        cuboid_body.vertices.push(vec3(ex, ey, ez));
        cuboid_body.vertices.push(vec3(-ex, ey, ez));

        cuboid_body
    }

    pub fn set_sleep(&mut self, b_sleep: bool){
        self.rigid_body.b_sleep = b_sleep;
    }

    pub fn ready_sleep(&mut self, curr_time: f32){
        if self.rigid_body.b_sleep == true{
            if curr_time - self.sleep_time > 0.35{
                self.rigid_body.b_static = true;
            }
        }
    }
}
impl Simulate for RigidBody{
    fn step(&mut self, dt: f32, acceleration: Vec3){
        self.dt = dt;
        if self.inv_mass == 0.0 {return;}
        if self.b_static {return;}
        //self.prev_vel = self.velocity;
        self.prev_pos = self.position;
        self.velocity += acceleration * dt;
        self.position += self.velocity * dt;


        self.prev_quaternion = self.quaternion;
        let mut d_rot = Quat::from_xyzw(self.angular_velocity.x,
                                        self.angular_velocity.y,
                                        self.angular_velocity.z,
                                   0.0);

        d_rot = d_rot.mul_quat(self.quaternion);

        self.quaternion.x += 0.5 * dt * d_rot.x;
        self.quaternion.y += 0.5 * dt * d_rot.y;
        self.quaternion.z += 0.5 * dt * d_rot.z;
        self.quaternion.w += 0.5 * dt * d_rot.w;
        self.quaternion= self.quaternion.normalize();
        self.inv_quaternion = self.quaternion.inverse();
    }

    fn update_vel(&mut self, dt:f32) {
        if self.inv_mass == 0.0 {return;}

        self.velocity = (self.position - self.prev_pos) / dt;

        let d_rot = self.quaternion.mul_quat(self.prev_quaternion.inverse());

        self.angular_velocity = vec3(d_rot.x * 2.0 / dt,
        d_rot.y * 2.0 / dt,
        d_rot.z * 2.0 / dt);

        if d_rot.w < 0.0{
            self.angular_velocity = self.angular_velocity.neg();
        }

        self.velocity *=  0.0_f32.max(1.0 - self.damping * dt);

    }

    fn _apply_correction(&mut self, corr: Vec3, pos: Vec3) {
        if self.inv_mass == 0.0 {return;}
        if self.b_static {return;}

        self.position += corr * self.inv_mass;

        let mut d_w = pos - self.position;

        d_w = d_w.cross(corr);

        d_w = self.inv_quaternion.mul_vec3(d_w);

        d_w = d_w.mul(self.inv_inertia);

        d_w = self.quaternion.mul_vec3(d_w);


        let mut d_rot = Quat::from_xyzw(d_w.x, d_w.y, d_w.z, 0.0);

        d_rot = d_rot.mul_quat(self.quaternion);


        self.quaternion.x += 0.5 * d_rot.x;
        self.quaternion.y += 0.5 * d_rot.y;
        self.quaternion.z += 0.5 * d_rot.z;
        self.quaternion.w += 0.5 * d_rot.w;
        self.quaternion= self.quaternion.normalize();
        self.inv_quaternion = self.quaternion.inverse();
    }

    fn apply_correction(&mut self, compliance: f32, corr: Vec3, pos: Vec3, other_body:  Option<Mut<CuboidBody>>, other_pos: Vec3,) -> f32 {
        if corr.length() == 0.0 {return 0.0;}

        let c = corr.length();

        let mut normal = corr.normalize();

        let mut w = self.get_mass_inv(normal, pos);
        if let Some(ref other_body1) = other_body{
            w += other_body1.get_mass_inv(normal, other_pos);
        }


        if w == 0.0 {
            return 0.0;
        }

        let alpha = compliance / self.dt / self.dt;

        let lambda = -c / (w + alpha);

        normal *= -lambda;

        self._apply_correction(normal, pos);

        if let Some(mut other_body_ref) = other_body{
            normal *= -1.0;
            other_body_ref._apply_correction(normal, other_pos);
        }

        lambda / self.dt / self.dt

    }


    fn get_position(&mut self) -> Vec3 {
        self.position
    }

    fn get_velocity(&self) -> Vec3 {
        self.velocity
    }

    fn get_quat(&mut self) -> Quat {
        self.quaternion
    }

    fn get_angule_vel(&self) -> Vec3 {
        self.angular_velocity
    }

    fn get_mass(&self) -> f32 {
        self.mass
    }

    fn get_mass_inv(&self, normal: Vec3, pos: Vec3) -> f32 {
        if self.inv_mass == 0.0 {return 0.0;}

        let mut rn = pos -self.position;
        rn = rn.cross(normal);
        rn = self.inv_quaternion.mul_vec3(rn);

        let mut w = rn.x * rn.x * self.inv_inertia.x +
            rn.y * rn.y  * self.inv_inertia.y +
            rn.z * rn.z * self.inv_inertia.z;

        w += self.inv_mass;
        w
    }



    fn local_to_world(&self, local_pos: Vec3) -> Vec3 {
        let world_pos =  self.position +  self.quaternion * local_pos;
        world_pos
    }

    fn world_to_local(&self, world_pos: Vec3) -> Vec3 {
        let local_pos =  self.inv_quaternion * (world_pos - self.position);
        local_pos
    }

    fn get_inv_inertia(&self) -> Vec3{
        self.inv_inertia
    }

    fn get_inertia(&self) -> Mat3 {
        self.inertia
    }

    fn get_centroid_world_pos(&self) -> Vec3 {
        self.centroid + self.position
    }
}
impl Simulate for CuboidBody {

    fn step(&mut self, dt: f32, acceleration: Vec3){
        self.rigid_body.step(dt, acceleration)
    }

    fn update_vel(&mut self, dt: f32) {
        self.rigid_body.update_vel(dt)
    }

    fn _apply_correction(&mut self, corr: Vec3, pos: Vec3) {
        self.rigid_body._apply_correction(corr, pos)
    }

    fn apply_correction(&mut self, compliance: f32, corr: Vec3, pos: Vec3, other_body:  Option<Mut<CuboidBody>>, other_pos: Vec3) -> f32 {
        self.rigid_body.apply_correction(compliance, corr, pos, other_body, other_pos)
    }




    fn get_position(&mut self) -> Vec3 {
        self.rigid_body.get_position()
    }

    fn get_velocity(&self) -> Vec3 {
        self.rigid_body.get_velocity()
    }

    fn get_quat(&mut self) -> Quat {
        self.rigid_body.get_quat()
    }

    fn get_angule_vel(&self) -> Vec3 {
        self.rigid_body.get_angule_vel()

    }

    fn get_mass(&self) -> f32 {
        self.rigid_body.get_mass()

    }

    fn get_mass_inv(&self, normal: Vec3, pos: Vec3) -> f32 {
        self.rigid_body.get_mass_inv(normal, pos)

    }



    fn local_to_world(&self, local_pos: Vec3) -> Vec3 {
        self.rigid_body.local_to_world(local_pos)
    }

    fn world_to_local(&self, world_pos: Vec3) -> Vec3 {
        self.rigid_body.world_to_local(world_pos)
    }

    fn get_inv_inertia(&self) -> Vec3 {
        self.rigid_body.get_inv_inertia()
    }

    fn get_inertia(&self) -> Mat3 {
        self.rigid_body.get_inertia()
    }

    fn get_centroid_world_pos(&self) -> Vec3 {
        self.rigid_body.get_centroid_world_pos()
    }
}

