use crate::comp::rigidbody::{CuboidBody, Simulate};
use bevy::prelude::*;
#[derive(Component)]
pub struct DistanceConstraint {
    body0: Entity,
    body1: Option<Entity>,
    world_pos0: Vec3,
    world_pos1: Vec3,
    local_pos0: Vec3,
    local_pos1: Vec3,
    distance: f32,
    compliance: f32,
    unilateral: bool,
    corr: Vec3,
}

impl DistanceConstraint{
    pub fn new(
        body0: Entity,
        body1: Option<Entity>,
        pos0: Vec3,
        pos1: Vec3,
        distance: f32,
        compliance: f32,
        unilateral: bool,
        query: &mut Query<(&mut CuboidBody, &mut Transform)>
    ) -> Self {
        let mut local_pos0 = pos0;
        let mut local_pos1 = pos1;
        if let Ok((cuboid_0, _transform)) = query.get(body0) {
            local_pos0 = cuboid_0.world_to_local(pos0);
        }

        if let Some(ref body_1_ref) = body1 {
            if let Ok((cuboid_1, _transform)) = query.get(*body_1_ref) {
                local_pos1 = cuboid_1.world_to_local(pos1);
            }
        }

        Self {
            body0,
            body1,
            world_pos0: pos0,
            world_pos1: pos1,
            local_pos0,
            local_pos1,
            distance,
            compliance,
            unilateral,
            corr: Vec3::ZERO,
        }
    }

    pub fn solve(&mut self, query: &mut Query<(Entity, &mut CuboidBody, &mut Transform)>) {
        let mut cuboid_0_ins = None;
        let mut cuboid_1_ins = None;

        for (entity, cuboid_body, _transform) in query.iter_mut() {
            if entity == self.body0 {
                cuboid_0_ins = Some(cuboid_body);
            }
            else{
                if let Some(body_1_ref) = self.body1 {
                    if entity == body_1_ref {
                        self.world_pos1 = cuboid_body.local_to_world(self.local_pos1);
                        cuboid_1_ins = Some(cuboid_body);
                    }
                }
            }

        }
        if cuboid_0_ins.is_none() {return;}
        let mut cuboid_0_ins = cuboid_0_ins.unwrap();


        self.world_pos0 = cuboid_0_ins.local_to_world(self.local_pos0);

        self.corr = self.world_pos1 - self.world_pos0;
        let distance = self.corr.length();
        self.corr = self.corr.normalize();

        if self.unilateral && distance <= self.distance {
            return;
        }

        self.corr *= distance - self.distance;

        {
            let _force = cuboid_0_ins.apply_correction(self.compliance, self.corr, self.world_pos0, cuboid_1_ins, self.world_pos1);
            let _elongation = (distance - self.distance).round();
            //print!("force: {} elongation: {} ", force, elongation);
        }


    }
    pub fn show_line(&mut self, gizmos: &mut Gizmos,) {
        gizmos.line(
            self.world_pos0,      // 起点
            self.world_pos1,     // 终点
            Color::srgb_u8(255, 0, 0),      // 颜色
        );
    }
}



