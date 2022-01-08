mod core;

use sdl2::rect::Rect;
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

    fn render(&mut self, ctx: &mut core::GameContext) -> core::FerResult {
        /* TODO: Implement Graphics Module */
        /* NOTE: PlaceHolder */
        ctx.canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
        ctx.canvas.fill_rect(Rect::new(0, 0, 64, 64)).map_err(core::FerError::SdlCanvasError)?;
        ctx.canvas.set_draw_color(sdl2::pixels::Color::BLACK);

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

    fn render(&mut self, ctx: &mut core::GameContext) -> core::FerResult {
        /* TODO: Implement Graphics Module */
        /* NOTE: PlaceHolder */
        ctx.canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 255));
        ctx.canvas.fill_rect(Rect::new(400, 300, 248, 128)).map_err(core::FerError::SdlCanvasError)?;
        ctx.canvas.set_draw_color(sdl2::pixels::Color::BLACK);

        Ok(())
    }
}
