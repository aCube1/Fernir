use std::collections::HashMap;
use sdl2::render;
use crate::core::error::{FerError, FerResult};

pub trait Scene {
    fn load(&mut self);
    fn update(&mut self, _dt: f64);
    fn render(&mut self, _canvas: &mut render::WindowCanvas) -> Result<(), String>;

    fn at_remove(&mut self) {}
}

pub struct SceneManager {
    active_scene: &'static str,
    scenes: HashMap<&'static str, Box<dyn Scene>>,
}

#[allow(dead_code)]
impl SceneManager {
    pub fn new() -> Self {
        Self {
            active_scene: "",
            scenes: HashMap::new(),
        }
    }

    pub fn add_scene<S>(&mut self, name: &'static str, mut scene: S) -> FerResult
    where
        S: Scene + 'static,
    {
        if self.scenes.contains_key(name) {
            return Err(FerError::SceneError("Scene Name Has Already Been Taken!"));
        }

        scene.load();
        self.scenes.insert(name, Box::new(scene));
        println!("Scene: {} Created Succefully", name);

        Ok(())
    }

    pub fn remove_scene(&mut self, name: &str) -> FerResult {
        if !self.scenes.contains_key(name) || name == "MAIN" {
            return Err(FerError::SceneError("Cannot Delete Inexistent/MAIN Scene"));
        }

        let mut scene = self.scenes.remove(name).unwrap();
        scene.at_remove();

        Ok(())
    }

    pub fn set_active_scene(&mut self, name: &'static str) -> FerResult<&'static str> {
        if !self.scenes.contains_key(name) {
            return Err(FerError::SceneError("Scene Doesn't Exist!"))
        }

        let last_scene = self.active_scene;
        self.active_scene = name;

        Ok(last_scene)
    }

    pub fn update(&mut self, dt: f64) -> Result<(), String> {
        let scene = self.scenes.get_mut(self.active_scene).unwrap().as_mut();
        scene.update(dt);

        Ok(())
    }

    pub fn render(&mut self, canvas: &mut render::WindowCanvas) -> Result<(), String> {
        let scene = self.scenes.get_mut(self.active_scene).unwrap().as_mut();
        scene.render(canvas)?;

        Ok(())
    }
}

impl Drop for SceneManager {
    fn drop(&mut self) {
        for (_, mut value) in self.scenes.drain() {
            value.at_remove();
        }
    }
}
