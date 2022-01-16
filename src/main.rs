use ggez::*;

mod game;
use game::Game;

fn main() {
    let (mut ctx, event_loop) = Game::setup()
        .expect("Cannot Setup Game");

    let game = Game::new(&mut ctx);

    event::run(ctx, event_loop, game);
}
