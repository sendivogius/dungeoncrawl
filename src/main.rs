use crate::prelude::*;

mod map;
mod player;
mod map_builder;

mod prelude {
    pub use bracket_lib::prelude::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;

    pub use crate::map::*;
    pub use crate::player::*;
    pub use crate::map_builder::*;
}

struct State {
    map: Map,
    player: Player,
}

impl State {
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let mb = MapBuilder::new(&mut rng);
        Self {
            map: mb.map,
            player: Player::new(mb.rooms[0].center()),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.player.update(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx)
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .build()?;
    main_loop(context, State::new())
}
