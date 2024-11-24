use bevy::app::{App, Plugin};
use bevy::input::ButtonInput;
use bevy::prelude::{Event, EventWriter, KeyCode, PreUpdate, Res};

#[derive(Event)]
pub struct MyEvent {
    pub message: String,
}
pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, handle_input);
    }
}

fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut event_writer: EventWriter<MyEvent>
)
{
    let mut new_event = MyEvent {
        message: "".to_string(),
    };

    if keyboard_input.just_pressed(KeyCode::Digit1) ||
        keyboard_input.just_pressed(KeyCode::Numpad1){
        new_event.message = "scene1".parse().unwrap();
    }
    else if keyboard_input.just_pressed(KeyCode::Digit2) ||
        keyboard_input.just_pressed(KeyCode::Numpad2){
        new_event.message = "scene2".parse().unwrap();
    }
    else if keyboard_input.just_pressed(KeyCode::Digit3) ||
        keyboard_input.just_pressed(KeyCode::Numpad3){
        new_event.message = "scene3".parse().unwrap();
    }
    else if keyboard_input.just_pressed(KeyCode::KeyP)
    {
        new_event.message = "pause".parse().unwrap();
    }
    else if keyboard_input.pressed(KeyCode::KeyC)
    {
        new_event.message = "mass+".parse().unwrap();
    }
    else if keyboard_input.pressed(KeyCode::KeyV)
    {
        new_event.message = "mass-".parse().unwrap();
    }
    else if keyboard_input.pressed(KeyCode::ArrowUp)
    {
        new_event.message = "ArrowUp".parse().unwrap();
    }
    else if keyboard_input.pressed(KeyCode::ArrowDown)
    {
        new_event.message = "ArrowDown".parse().unwrap();
    }
    else if keyboard_input.pressed(KeyCode::ArrowLeft)
    {
        new_event.message = "ArrowLeft".parse().unwrap();
    }
    else if keyboard_input.pressed(KeyCode::ArrowRight)
    {
        new_event.message = "ArrowRight".parse().unwrap();
    }
    else if keyboard_input.pressed(KeyCode::PageUp)
    {
        new_event.message = "PageUp".parse().unwrap();
    }
    else if keyboard_input.pressed(KeyCode::PageDown)
    {
        new_event.message = "PageDown".parse().unwrap();
    }
    else {
        return;
    }
    event_writer.send(new_event);
}