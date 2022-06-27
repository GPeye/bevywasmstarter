use crate::game::Log;
use bevy::prelude::*;
use wasm_bindgen::*;

#[wasm_bindgen]
#[cfg(target_arch = "wasm32")]
extern "C" {
    fn has_touch() -> bool;
}

#[wasm_bindgen]
extern "C" {
    fn pop_touch_event() -> Option<TouchEvent>;
}

fn pool_touch_system(
    windows: Res<Windows>,
    mut sprites: Query<(Entity, &mut Sprite), With<Clickable>>,
) {
    if let Some(window) = windows.get_primary() {
        while let Some(touch_event) = pop_touch_event() {
            let t = touch_event.type_();
            let touches: TouchList = touch_event.touches();

            for i in 0..touches.length() {
                if let Some(touch) = touches.get(i) {
                    let _id = touch.identifier();
                    let x = touch.client_x() as f32;
                    let y = window.height() as f32 - touch.client_y() as f32;
                    println!("{:?}, {:?}", x, y);
                    //mouse.pos_screen = Vec2::new(x, y);
                    //mouse.has_touch = true;
                }
            }

            if t == "touchstart" {
                Log("touch start");
                for (_, mut sprite) in sprites.iter_mut() {
                    let color = bevy::render::color::Color::rgb(0.5, 0.5, 0.0);
                    sprite.color = color;
                }
                //mouse_button_input.press(MouseButton::Left);
            } else if t == "touchend" {
                for (_, mut sprite) in sprites.iter_mut() {
                    let color = bevy::render::color::Color::rgb(0.7, 0.7, 0.7);
                    sprite.color = color;
                }
                Log("touch end");
                //mouse_button_input.release(MouseButton::Left);
            } else if t == "touchmove" {
            }
        }
    }
}

pub struct TouchPlugin;

impl Plugin for TouchPlugin {
    fn build(&self, app: &mut App) {
        //app.add_plugin(MousePlugin);
    }
}
