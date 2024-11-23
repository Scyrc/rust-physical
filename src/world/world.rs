use crate::comp::distance_joint::DistanceConstraint;
use crate::comp::rigidbody::{CuboidBody, Simulate};
use crate::ui::event::MyEvent;
use bevy::math::vec3;
use bevy::prelude::*;
use rand::Rng;

pub struct WorldPlugin;

#[derive(Resource)]
pub struct Setting{
    pub wind: Vec3,
    pub pause:bool,
}
impl Default for Setting {
    fn default() -> Self {
        Setting{
            wind: vec3(-8.0, 0.0, -8.0),
            pause: false,
        }
    }
}

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_world)
            .insert_resource(Setting::default()) // 插入资源
            .add_systems(FixedUpdate, tick)
            .add_systems(FixedPostUpdate, show)
            .add_event::<MyEvent>()
            .add_systems(Update, handle_event);

    }
}
fn handle_event(mut event_reader: EventReader<MyEvent>,
                mut setting: ResMut<Setting>,
                mut commands: Commands, // 用于操作实体
                mut query: Query<Entity, (Without<Text>, Without<Window>)>,
                mut meshes: ResMut<Assets<Mesh>>,
                mut materials: ResMut<Assets<StandardMaterial>>,
                mut query1: Query<(&mut CuboidBody, &mut Transform)>,
                time: Res<Time>)
{

    if event_reader.is_empty() {return;}
    let  wind_factor = 0.2;

    for event in event_reader.read(){
        let msg =  event.message.clone();
        match msg.as_str() {
            "scene1" => {
                clear_scene(&mut commands,  &mut meshes, &mut materials,&mut query);
                setting.wind = vec3(-8.0, 0.0, 8.0);
                scene_base(&mut commands, &mut meshes, &mut materials, &mut query1);
            }
            "scene2" => {
                clear_scene(&mut commands,  &mut meshes, &mut materials, &mut query);
                setting.wind = vec3(-8.0, 0.0, -8.0);
                scene_chain(&mut commands, &mut meshes, &mut materials, &mut query1);
            }
            "scene3" => {
                clear_scene(&mut commands,  &mut meshes, &mut materials, &mut query);
                setting.wind = vec3(0.0, 0.0, 8.0);

                scene_chain_timer(&mut commands, &mut meshes, &mut materials, &mut query1, &time);
            }
            "pause" => {
                setting.pause = !setting.pause;
            }
            "ArrowUp" => {
                setting.wind.z -= wind_factor;
            }
            "ArrowDown" => {
                setting.wind.z += wind_factor;
            }
            "ArrowLeft" => {
                setting.wind.x -= wind_factor;
            }
            "ArrowRight" => {
                setting.wind.x += wind_factor;
            }
            "PageUp" => {
                setting.wind.y += wind_factor;
            }
            "PageDown" => {
                setting.wind.y -= wind_factor;
            }
            _ => {}
        }
    }
}

fn clear_scene(commands: &mut Commands, // 用于操作实体
               meshes:  &mut ResMut<Assets<Mesh>>,
               materials:  &mut ResMut<Assets<StandardMaterial>>,
               query: &mut Query<Entity, (Without<Text>, Without<Window>)>,
)
{
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            color: Color::srgb_u8(255, 226, 201),
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands
        .spawn(Camera3dBundle {
            camera: Camera {
                order: 0, // 3D 摄像机优先级
                ..default()
            },
            transform: Transform::from_xyz(0.0, 2.0, 4.0)
                .looking_at(vec3(0.0, 2.0, 0.0), Vec3::Y),
            ..default()
        })
        .insert(crate::scene::camera::CameraController {
            speed: 5.0,
            sensitivity: 0.1,
            yaw: 0.0,
            pitch: 0.0,
        });

}

fn init_world(mut event_writer: EventWriter<MyEvent>, ) {
    let new_event = MyEvent {
        message: "scene2".to_string(),
    };
    event_writer.send(new_event);
}

fn scene_base(mut commands: &mut Commands,
           mut meshes:  &mut ResMut<Assets<Mesh>>,
           mut materials:  &mut ResMut<Assets<StandardMaterial>>,
           mut query:  &mut Query<(&mut CuboidBody, &mut Transform)>,
)
{
    let mut init_pos = vec3(-1.6, 2.6, 0.0);
    let origin_size = vec3(0.15, 0.15, 0.15);

    let mut init_size = origin_size;

    let mut connect_points:Vec<Vec3> = Vec::new();
    connect_points.push(vec3(0.5, 0.5, 0.5));
    connect_points.push(vec3(0.3, 0.5, 0.0));
    connect_points.push(vec3(0.5, 0.5, 0.3));
    connect_points.push(vec3(0.0, 0.0, 0.0));
    connect_points.push(vec3(-0.3, 0.5, 0.5));
    connect_points.push(vec3(-0.5, 0.5, 0.0));
    connect_points.push(vec3(-0.5, 0.5, -0.3));

    let num = connect_points.len();
    let x_diff = 0.4;
    let y_diff = 0.3;
    let mut _idx = 0;
    let mut rng = rand::thread_rng();
    for connect_point in connect_points.iter() {
        let mut box_origin_pos = init_pos;
        box_origin_pos.y += y_diff;
        if _idx < num / 2 {
            box_origin_pos.x -= x_diff;
        }
        else {
            box_origin_pos.x += x_diff;
        }
        let entity = add_cuboid_body(&mut commands, &mut meshes, &mut materials, box_origin_pos, init_size, false, 0.0);
        let  local_pos =  init_size * *connect_point;

        add_distance_joint(&mut commands, &mut meshes, &mut materials, entity, None, &mut query, init_pos, local_pos);
        init_pos.x += 0.5;
        if _idx < num / 2 {
            init_pos.y -= 0.5;
            init_size.x =  origin_size.x * rng.gen_range(0.7..2.6);
            init_size.y =  origin_size.y * rng.gen_range(0.7..2.6);
            init_size.z =  origin_size.z * rng.gen_range(0.7..2.6);

        }
        else {
            init_pos.y += 0.5;
            init_size.x =  origin_size.x * rng.gen_range(0.7..2.6);
            init_size.y =  origin_size.y * rng.gen_range(0.7..2.6);
            init_size.z =  origin_size.z * rng.gen_range(0.7..2.6);
        }
        _idx += 1;
    }
}

fn scene_chain(mut commands: &mut Commands,
           mut meshes: &mut ResMut<Assets<Mesh>>,
           mut materials: &mut ResMut<Assets<StandardMaterial>>,
           mut query: &mut Query<(&mut CuboidBody, &mut Transform)>,
)
{
    let mut box_size = vec3(0.1, 0.1, 0.1);
    let mut prev_y = 2.5;
    let mut box_pos = vec3(0.0, prev_y, 0.0);
    let dist = 0.2;
    let mut prev_size = 0.0;
    let mut prev_box: Option<Entity> = None;
    let mut rng = rand::thread_rng();

    for _level in 0..4 {
        prev_y = box_pos.y;
        box_pos.y -= dist + box_size.y;
        if _level % 2 == 0 {
            box_pos.x = 0.3;
        }
        else {
            box_pos.x = -0.3;
        }
        let entity = add_cuboid_body(&mut commands, &mut meshes, &mut materials, box_pos, box_size, false, 0.0);
        //println!("box_idx: {} y_pos:{} box_size:{}", _level,  box_pos.y,  box_size);
        let length = (prev_y - box_pos.y - prev_size * 0.5 - box_size.y * 0.5).abs() * 1.2;
        //println!("length: {} ", length);

        add_distance_joint_new(&mut commands, entity, prev_box, box_size.y, prev_size, length, _level, box_pos.y ,&mut query);

        prev_box = Some(entity);
        prev_size = box_size.y;
        box_size *= rng.gen_range(1.4..1.7); // 更新 boxSize
    }
}

fn scene_chain_timer(mut commands: &mut Commands,
               mut meshes: &mut ResMut<Assets<Mesh>>,
               mut materials: &mut ResMut<Assets<StandardMaterial>>,
               mut query: &mut Query<(&mut CuboidBody, &mut Transform)>,
                     time: &Res<Time>,
)
{
    let mut prev_x = 0.0;

    let mut box_size = vec3(0.1, 0.1, 0.1);
    let mut box_pos = vec3(prev_x, 3.0, -1.0);
    let dist = 0.2;
    let mut prev_size = 0.0;
    let mut prev_box = add_cuboid_body(&mut commands, &mut meshes, &mut materials, box_pos, box_size, true, time.elapsed_seconds());
    let mut rng = rand::thread_rng();
    for _level in 0..4 {
        prev_x = box_pos.x;
        box_pos.x += dist + box_size.x;
        // if _level % 2 == 0 {
        //     box_pos.x = 0.3;
        // }
        // else {
        //     box_pos.x = -0.3;
        // }
        let entity = add_cuboid_body(&mut commands, &mut meshes, &mut materials, box_pos, box_size, false, 0.0);
        let length = (box_pos.x - prev_x  - prev_size * 0.5 - box_size.x * 0.5).abs() * 1.2;
        //println!("length: {} ", length);

        add_distance_joint_new1(&mut commands, entity, prev_box, box_size.x, prev_size, length,&mut query);

        prev_box = entity;
        prev_size = box_size.x;
        box_size *= rng.gen_range(0.8..1.5);
    }
}

fn add_cuboid_body(commands: &mut Commands,
                   meshes: &mut ResMut<Assets<Mesh>>,
                   materials: &mut ResMut<Assets<StandardMaterial>>,
                   init_pos: Vec3,
                   init_size: Vec3,
                   b_static: bool,
                   spawn_time:f32,

) -> Entity
{
    let init_angle = vec3(0.0, 0.0, 0.0);

    let  mut cuboid_body = CuboidBody::new(init_pos, init_size, init_angle, 20.0);
    cuboid_body.set_sleep(b_static);
    cuboid_body.sleep_time = spawn_time;

    let entity_idx = commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(init_size.x, init_size.y, init_size.z)),
        material: materials.add(Color::srgb_u8(124, 144, 255)),
        transform: Transform::from_xyz(init_pos.x, init_pos.y, init_pos.z),
        ..default()
    }).insert(cuboid_body).id();
    entity_idx
}

fn add_distance_joint(commands: &mut Commands,
                      meshes: &mut ResMut<Assets<Mesh>>,
                      materials: &mut ResMut<Assets<StandardMaterial>>,
                      body: Entity,
                      body1: Option<Entity>,

                      query: &mut Query<(&mut CuboidBody, &mut Transform)>,

                      init_pos: Vec3,
                      loc_pos: Vec3,
)
{
    let y_len = 0.6;
    let mut target_pos = init_pos;
    target_pos.y += y_len;
    let joint_body = DistanceConstraint::new(body,
                                                 body1,
                                                 loc_pos,
                                                 target_pos,
                                                 y_len,
                                                 0.001,
                                                 true, query);


    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(0.03)),
        material: materials.add(Color::srgb_u8(0, 255, 0)),
        transform: Transform::from_xyz(target_pos.x, target_pos.y, target_pos.z),
        ..default()
    }).insert(joint_body);
}

fn add_distance_joint_new(commands: &mut Commands,
                                    body: Entity,
                                    body1: Option<Entity>,
                                    box_size: f32,
                                    box1_size: f32,
                                    dis: f32,
                                    idx: i32,
                                    y_pos: f32,
                                    query: &mut Query<(&mut CuboidBody, &mut Transform)>,

)
{
    let mut prev_pos = vec3(0.0, -0.5 * box1_size, 0.0);
    if idx == 0{
        prev_pos = vec3(0.0, y_pos + dis, 0.0);
    }
    let joint_body = DistanceConstraint::new(body,
                                                 body1,
                                                 vec3(0.4 * box_size ,0.5 * box_size, 0.0),
                                                 prev_pos,
                                                 dis,
                                                 0.001,
                                                 true, query);


    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    }).insert(joint_body);
}
fn add_distance_joint_new1(commands: &mut Commands,
                          body: Entity,
                          body1:Entity,
                          box_size: f32,
                           prev_size: f32,
                          dis: f32,
                          query: &mut Query<(&mut CuboidBody, &mut Transform)>,

)
{
    let prev_pos = vec3(0.5 * prev_size, 0.2 * prev_size, 0.0);
    let joint_body = DistanceConstraint::new(body,
                                             Some(body1),
                                             vec3(-0.5 * box_size ,0.2 * box_size, 0.0),
                                             prev_pos,
                                             dis,
                                             0.001,
                                             true, query);


    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    }).insert(joint_body);
}


fn tick(
    time: Res<Time>,
    setting: ResMut<Setting>,
    mut query: Query<(Entity, &mut CuboidBody, &mut Transform)>,
    mut joint_query: Query<&mut DistanceConstraint>
) {
    if setting.pause == true {return; }
    for (_entity, mut cuboid_body,   _transform )in query.iter_mut() {
        cuboid_body.ready_sleep(time.elapsed_seconds());
    }
    let delta_time = time.delta_seconds();
    let num_sub_steps = 8;
    let mut s_dt = delta_time / num_sub_steps as f32;
    s_dt /= 1.0;
    let g = vec3(0.0, -9.80, 0.0) + setting.wind;
    for _i in 0..num_sub_steps {

        for (_entity, mut cuboid_body,  _transform )in query.iter_mut() {
            cuboid_body.step(s_dt, g);
        }

        for mut distance_joint in joint_query.iter_mut() {
            distance_joint.solve(&mut query);
        }

        for (_entity, mut cuboid_body,  _transform )in query.iter_mut() {
            cuboid_body.update_vel(s_dt);
        }
    }



}

fn show(
    mut gizmos: Gizmos,
    mut query: Query<(Entity, &mut CuboidBody, &mut Transform)>,
    mut joint_query: Query<&mut DistanceConstraint>
) {
    for mut distance_joint in joint_query.iter_mut() {
        distance_joint.show_line(&mut gizmos);
    }

    for (_entity, mut cuboid_body,  mut transform )in query.iter_mut() {
        transform.translation = cuboid_body.get_position();
        transform.rotation  = cuboid_body.get_quat();
    }

}