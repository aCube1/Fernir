mod core;

use sdl2::{render, rect, pixels};
use crate::core::scene;

struct MainScene {
    rect_x: i32,
    rect_y: i32,
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
            rect_x: 0,
            rect_y: 300,
        }
    }
}

impl scene::Scene for MainScene {
    fn load(&mut self) {}
    fn update(&mut self, dt: f64) {
        self.rect_x += (200.0 * dt) as i32;
    }
    fn render(&mut self, canvas: &mut render::WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(pixels::Color::RGB(255, 255, 255));
        canvas.draw_rect(rect::Rect::new(self.rect_x, self.rect_y, 32, 32))?;
        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));

        Ok(())
    }
}
