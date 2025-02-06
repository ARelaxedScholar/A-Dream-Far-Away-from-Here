#![warn(clippy::all, clippy::pedantic)]
mod map;
mod map_builder;
mod player;
mod prelude;

// Prelude
use prelude::*;

pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;

struct State {
    map: Map,
    player: Player,
}

impl State {
    fn new() -> Self {
        let generator = HandsOnGenerator {};
        let map_builder = MapBuilder::new(generator);

        Self {
            map: map_builder.map,
            player: Player::new(map_builder.player_start),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.player.update(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx);
    }
}

fn main() -> BError {
    // let context = BTermBuilder::new()
    //     .with_title("A Dream Far Away from Here")
    //     .with_fps_cap(30.0)
    //     .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
    //     .with_tile_dimensions(32, 32)
    //     .with_resource_path("resources/")
    //     .with_font("dungeonfont.png", 32, 32)
    //     .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
    //     .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
    //     .build()?;
    let context = BTermBuilder::simple80x50().build()?;
    main_loop(context, State::new())
}
