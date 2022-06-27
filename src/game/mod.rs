mod player;
pub use player::*;

mod log;
pub use log::*;

mod input;
pub use input::*;

#[cfg(target_arch = "wasm32")]
mod wasm;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputPlugin);

        app.add_plugin(PlayerPlugin);

        #[cfg(target_arch = "wasm32")]
        app.add_plugin(wasm::WASMPlugin);
    }
}
