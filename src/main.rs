mod core;

use crate::core::scene;

struct MainScene {}

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
        MainScene {}
    }
}

impl scene::Scene for MainScene {
    fn load(&mut self) -> core::FerResult {
        Ok(())
    }

    fn update(&mut self, ctx: &core::GameContext) -> core::FerResult {
        let dt = ctx.timer.get_dt();
        println!("DT: {}", dt);

        Ok(())
    }

    fn render(&mut self, _ctx: &core::GameContext) -> core::FerResult {
        Ok(())
    }
}
