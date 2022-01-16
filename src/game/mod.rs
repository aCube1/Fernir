use ggez::{
    Context, ContextBuilder, GameResult,
    event::{EventLoop, EventHandler},
};

mod config;

pub struct Game {}

impl Game {
    pub fn setup() -> GameResult<(Context, EventLoop<()>)> {
        let context_builder = ContextBuilder::new("Fernir", "aCube");

        config::load_config(context_builder)
            .expect("Cannot Load Game Configuration!")
            .build()
    }

    pub fn new(_ctx: &mut Context) -> Self {
        Self {}
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
}
