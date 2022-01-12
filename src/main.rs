mod core;

use crate::core::scene;
use crate::core::graphics;

struct MainScene {
    sprite: Option<graphics::SpriteId>,
}

fn main() {
    let main_scene = MainScene::new();

    core::CoreBuilder::new()
        .build(main_scene)
        .expect("Cannot Create Core!")
        .run()
        .expect("Core Exitted!");
}

impl MainScene {
    pub fn new() -> MainScene {
        MainScene { sprite: None }
    }
}

impl scene::Scene for MainScene {
    fn load(&mut self) -> core::FerResult<scene::Transition> {
        Ok(scene::Transition::None)
    }

    fn update(&mut self, ctx: &mut core::GameContext) -> core::FerResult<scene::Transition> {
        let _dt = ctx.timer.get_dt();
        let _fps = ctx.timer.get_fps();
        //println!("DT: {} | FPS: {}", dt, fps);

        if self.sprite.is_none() {
            self.sprite = Some(ctx.graphics.new_sprite("assets/test.jpg")?);
        }

        Ok(scene::Transition::None)
    }

    fn render(&mut self, ctx: &mut core::GameContext) -> core::FerResult {
        ctx.graphics.render_sprite(self.sprite.as_ref().unwrap(), 0, 0, 400, 600)?;
        Ok(())
    }
}

