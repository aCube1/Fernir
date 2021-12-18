mod core;

use sdl2::render;
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
    fn load(&mut self) {}
    fn update(&mut self, _dt: f64) {}
    fn render(&mut self, _render: &render::WindowCanvas) {}
}
