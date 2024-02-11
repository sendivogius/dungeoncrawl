use crate::prelude::*;

mod player_input;
mod map_renderer;
mod entity_renderer;
mod collisions;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(map_renderer::map_render_system())
        .add_system(entity_renderer::entity_render_system())
        .add_system(collisions::collisions_system())
        .build()
}