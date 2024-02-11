use crate::prelude::*;
use std::fmt::Pointer;

mod collisions;
mod end_turn;
mod entity_renderer;
mod map_renderer;
mod player_input;
mod random_move;
mod movement;
mod hud;
mod tooltips;

pub fn build_monster_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(random_move::random_move_system())
        .flush()
        .add_system(movement::movement_system())
        .flush()
        .add_system(map_renderer::map_render_system())
        .add_system(entity_renderer::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(tooltips::tooltips_system())
        .add_system(end_turn::end_turn_system())
        .build()
}

pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .flush()
        .add_system(map_renderer::map_render_system())
        .add_system(entity_renderer::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(tooltips::tooltips_system())
        .build()
}

pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(movement::movement_system())
        .flush()
        .add_system(collisions::collisions_system())
        .flush()
        .add_system(map_renderer::map_render_system())
        .add_system(entity_renderer::entity_render_system())
        .add_system(end_turn::end_turn_system())
        .add_system(hud::hud_system())
        .add_system(tooltips::tooltips_system())
        .build()
}
