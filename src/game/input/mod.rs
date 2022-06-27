mod mouse;
pub use mouse::*;

use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MousePlugin);
    }
}
