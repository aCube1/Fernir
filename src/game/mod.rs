use ggez::timer;
use ggez::{
    event::{EventHandler, EventLoop},
    graphics::{self, Color, DrawParam, DrawMode, Mesh, Rect},
    input::keyboard::{KeyCode, KeyMods},
    Context, ContextBuilder, GameResult,
};
use hecs::*;

mod components;
mod config;
mod spawner;
mod systems;

pub struct Game {
    world: World,
}

impl Game {
    pub fn setup() -> GameResult<(Context, EventLoop<()>)> {
        let context_builder = ContextBuilder::new("Fernir", "aCube");

        config::load_config(context_builder)
            .expect("Cannot Load Game Configuration!")
            .build()
    }

    pub fn new(_ctx: &mut Context) -> GameResult<Self> {
        let mut world = World::new();
        spawner::spawn_player(&mut world);

        Ok(Self {
            world,
        })
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        systems::movement::movement(&mut self.world, &timer::delta(ctx));
        systems::movement::apply_friction(&mut self.world);
        systems::movement::apply_velocity(&mut self.world);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut query = self.world.query::<&components::Position>();

        graphics::clear(ctx, Color::WHITE);
        for (_e, pos) in query.iter() {
            let rect = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(pos.x, pos.y, 32.0, 32.0),
                Color::RED,
            ).expect("Can't Create Rect For Drawing");


            let draw_param = DrawParam::default();

            graphics::draw(ctx, &rect, draw_param)
                .expect("That's Not Normal");
        }
        graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        systems::movement::input(&mut self.world, &keycode);
    }
}
