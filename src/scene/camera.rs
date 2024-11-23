use std::f32::consts::PI;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
pub struct CameraControlPlugin;

impl Plugin for CameraControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera_movement);
    }
}

#[derive(Component)]
pub struct CameraController {
    pub speed: f32, // 移动速度
    pub sensitivity: f32, // 鼠标灵敏度
    pub yaw: f32,
    pub pitch: f32,
}
fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut mouse_events: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut CameraController)>,
) {
    for (mut transform, mut controller) in query.iter_mut() {
        // 移动相机
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.z -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.z += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Space) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ShiftLeft) {
            direction.y -= 1.0;
        }

        // 更新相机位置
        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
            transform.translation += direction * controller.speed * time.delta_seconds();
        }

        // 旋转相机（鼠标控制）
        let mut mouse_delta = Vec2::ZERO;
        if mouse_button.pressed(MouseButton::Right) {
            for mouse_event in mouse_events.read() {
                mouse_delta += mouse_event.delta;
            }
        } else {
            mouse_events.clear();
        }
        const RADIANS_PER_DOT: f32 = 1.0 / 180.0;
        if mouse_delta != Vec2::ZERO {
            controller.pitch = (controller.pitch - mouse_delta.y * RADIANS_PER_DOT * controller.sensitivity)
                .clamp(-PI / 2., PI / 2.);
            controller.yaw -= mouse_delta.x * RADIANS_PER_DOT * controller.sensitivity;
            transform.rotation =
                Quat::from_euler(EulerRot::ZYX, 0.0, controller.yaw, controller.pitch);
        }

    }
}
