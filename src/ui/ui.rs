use crate::world::world::Setting;
use bevy::ecs::system::Query;
use bevy::prelude::*;
use bevy::{
    color::palettes::css::GOLD,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, text_update_system);
    }
}

#[derive(Component)]
struct CustomText;

// A unit struct to help identify the color-changing Text component
#[derive(Component)]
pub(crate) struct ColorText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    let tips_text = "press \n\
    '1' to enter  base scene.\n\
    '2' to enter chain scene.\n\
    '3' to enter fall chain scene.\n\
    'p' to pause/unpause world.\n\
    ---------------------------------------\n\
    press & hold\n\
    'c'/'v' to increase/decrease density.\n\
    'up,down' to change wind in z.\n\
    'left,right' to change wind in x.\n\
    'pageup,pagedown' to change wind in y.\n\
    'w,a,s,d,q,e,mouse right button' to control camera.\n\
    ";

    // UI camera
    commands.spawn(Camera2dBundle::default());
    // Text with one section
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            tips_text,
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 22.0,
                ..default()
            },
        ) // Set the justification of the Text
            .with_text_justify(JustifyText::Left)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(1.0),
                left: Val::Px(1.0),
                ..default()
            }),
        ColorText,
    ));

    // Text with multiple sections
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "fps:",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    ..default()
                }
            ),
            TextSection::from_style(
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 30.0,
                    color: GOLD.into(),
                }
            ),
        ]).with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }),
        CustomText,
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "density factor:",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    ..default()
                }
            ),
            TextSection::from_style(
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 30.0,
                    color: GOLD.into(),
                }
            ),
        ]).with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(35.0),
            right: Val::Px(5.0),
            ..default()
        }),
        CustomText,
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "wind:",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    ..default()
                }
            ),
            TextSection::from_style(
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 30.0,
                    color: GOLD.into(),
                }
            ),
        ]).with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(65.0),
            right: Val::Px(5.0),
            ..default()
        }),
        CustomText,
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "scene",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::srgb_u8(0, 255, 0),
                }
            ),
        ]).with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(45.0),
            left: Val::Px(5.0),
            ..default()
        }),
        CustomText
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "No Collision Version.",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    ..default()
                }
            ),
        ]).with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
    ));
}

// pub fn init_ui(time: Res<Time>, mut query: Query<&mut Text, With<ColorText>>) {
//     for mut text in &mut query {
//         let seconds = time.elapsed_seconds();
//
//         // Update the color of the first and only section.
//         text.sections[0].style.color = Color::srgb(
//             (1.25 * seconds).sin() / 2.0 + 0.5,
//             (0.75 * seconds).sin() / 2.0 + 0.5,
//             (0.50 * seconds).sin() / 2.0 + 0.5,
//         );
//     }
// }

fn text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<CustomText>>,
    // mut query_wind: Query<&mut Text, With<WindText>>,
    setting: ResMut<Setting>,
) {
    for mut text in &mut query {
        if text.sections[0].value.starts_with("fps")
        {
            if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(value) = fps.smoothed() {
                    text.sections[1].value = format!("{value:.2}");
                }
            }
        }
        else  if text.sections[0].value.starts_with("wind")
        {
            let wind = setting.wind;
            let x= wind.x;
            let y= wind.y;
            let z= wind.z;

            text.sections[1].value = format!("({x:.1}, {y:.1}, {z:.1})");
        }

        else  if text.sections[0].value.starts_with("density")
        {
            let mass_factor = setting.mass_factor;
            text.sections[1].value = format!("{mass_factor:.2}");
        }

        else  if text.sections[0].value.contains("scene")
        {
            let scene_id = setting.scene_id;
            let mut scene_name = "";
            match scene_id {
                0 => { scene_name = "base scene."}
                1 => { scene_name = "chain scene."}
                2 => { scene_name = "fall chain scene."}
                _ => {}
            }
            text.sections[0].value = scene_name.parse().unwrap();
        }
    }
}