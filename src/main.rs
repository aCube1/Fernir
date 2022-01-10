mod core;

use crate::core::scene;

struct MainScene {
    time: f32,
}

/* NOTE: PlaceHolder */
struct NepScene {
    time: f32,
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
        MainScene {
            time: 0.0,
        }
    }
}

/* NOTE: PlaceHolder */
impl NepScene {
    pub fn new() -> NepScene {
        NepScene {
            time: 0.0,
        }
    }
}

impl scene::Scene for MainScene {
    fn load(&mut self) -> core::FerResult<scene::Transition> {
        Ok(scene::Transition::Push("NEP", Box::new(NepScene::new())))
    }

    fn update(&mut self, ctx: &mut core::GameContext) -> core::FerResult<scene::Transition> {
        let dt = ctx.timer.get_dt();
        let _fps = ctx.timer.get_fps();
        //println!("DT: {} | FPS: {}", dt, fps);

        /* NOTE: PlaceHolder */
        self.time += dt as f32;
        if self.time >= 5.0 {
            self.time = 0.0;
            Ok(scene::Transition::Switch("NEP"))
        } else {
            Ok(scene::Transition::None)
        }
    }

    fn render(&mut self, _ctx: &mut core::GameContext) -> core::FerResult {
        Ok(())
    }
}

impl scene::Scene for NepScene {
    fn load(&mut self) -> core::FerResult<scene::Transition> {
        Ok(scene::Transition::None)
    }

    fn update(&mut self, ctx: &mut core::GameContext) -> core::FerResult<scene::Transition> {
        let dt = ctx.timer.get_dt();
        let _fps = ctx.timer.get_fps();
        //println!("DT: {} | FPS: {}", dt, fps);

        /* NOTE: PlaceHolder */
        self.time += dt as f32;
        if self.time >= 5.0 {
            self.time = 0.0;
            Ok(scene::Transition::Switch("MAIN"))
        } else {
            Ok(scene::Transition::None)
        }
    }

    fn render(&mut self, _ctx: &mut core::GameContext) -> core::FerResult {
        Ok(())
    }
}
