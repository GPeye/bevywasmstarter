use crate::game::{Clickable, Log};
use bevy::{
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    prelude::*,
};

pub fn mouse_click_system(
    mouse_button_input: Res<Input<MouseButton>>,
    mut sprites: Query<(Entity, &mut Sprite), With<Clickable>>,
) {
    if mouse_button_input.pressed(MouseButton::Left) {
        Log("left mouse currently pressed");
        for (_, mut sprite) in sprites.iter_mut() {
            let color = bevy::render::color::Color::rgb(0.5, 0.5, 0.0);
            sprite.color = color;
        }
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        Log("left mouse just pressed");
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        Log("left mouse just released");
        for (_, mut sprite) in sprites.iter_mut() {
            let color = bevy::render::color::Color::rgb(0.7, 0.7, 0.7);
            sprite.color = color;
        }
    }
}

pub fn print_mouse_events_system(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
) {
    for event in mouse_button_input_events.iter() {
        //uncomment for a log of mouse event output
        //Log(&format_args!("{:?}", event).to_string());
    }

    for event in mouse_motion_events.iter() {
        //uncomment for a log of mouse event output
        //Log(&format_args!("{:?}", event).to_string());
    }

    for event in cursor_moved_events.iter() {
        //uncomment for a log of mouse event output
        //Log(&format_args!("{:?}", event).to_string());
    }

    for event in mouse_wheel_events.iter() {
        //uncomment for a log of mouse event output
        //Log(&format_args!("{:?}", event).to_string());
    }
}

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(mouse_click_system)
            .add_system(print_mouse_events_system);
    }
}
