#![allow(clippy::uninlined_format_args)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]

pub use comfy::*;
pub use grids::*;

mod assets;
mod game;
mod lighting;
mod mobs;
mod player;
mod utils;

pub use crate::game::*;
pub use crate::lighting::*;
pub use crate::mobs::*;
pub use crate::player::*;
pub use crate::utils::*;

simple_game!("BITMOB", GameState, setup, game_update);

fn setup(_state: &mut GameState, _c: &mut EngineContext) {
    crate::assets::load_assets();
}
