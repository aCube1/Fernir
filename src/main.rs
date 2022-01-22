use ggez::*;

mod game;
use game::Game;

fn main() {
    env_logger::init();

    let (mut ctx, event_loop) = Game::setup().expect("Cannot Setup Game");

    let game = Game::new(&mut ctx).expect("Cannot Create Game Handler");

    event::run(ctx, event_loop, game);
}
