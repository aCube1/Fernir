use sdl2::render;
use std::collections::HashMap;

pub trait Scene {
    fn load(&mut self);
    fn update(&mut self, _dt: f64);
    fn render(&mut self, _canvas: &render::WindowCanvas);
}

pub struct SceneManager {
    active_scene: &'static str,
    scenes: HashMap<&'static str, Box<dyn Scene>>,
}

impl SceneManager {
    pub fn new() -> Self {
        Self {
            active_scene: "",
            scenes: HashMap::new(),
        }
    }

    pub fn add_state<S>(&mut self, name: &'static str, mut scene: S) -> Result<(), String>
    where
        S: Scene + 'static,
    {
        if self.scenes.contains_key(name) {
            return Err("Scene Name Has Already Been Taken!".into());
        }

        scene.load();
        self.scenes.insert(name, Box::new(scene));
        println!("Scene: {} Created Succefully", name);

        Ok(())
    }

    pub fn set_active_scene(&mut self, name: &'static str) -> Result<&'static str, String> {
        if !self.scenes.contains_key(name) {
            return Err("Scene Doesn't Exist!".into())
        }

        let last_scene = self.active_scene;
        self.active_scene = name;

        Ok(last_scene)
    }

    pub fn update_scene(&self, dt: f64) {
        
    }
}
