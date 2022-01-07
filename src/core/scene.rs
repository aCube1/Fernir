use std::collections::HashMap;
use crate::core::{self, error::{FerError, FerResult}};

pub trait Scene {
    fn load(&mut self) -> FerResult;
    fn update(&mut self, _ctx: &core::GameContext) -> FerResult;
    fn render(&mut self, _ctx: &core::GameContext) -> FerResult;

    fn at_remove(&mut self) {}
}

pub struct SceneManager {
    active_scene: &'static str,
    scenes: HashMap<&'static str, Box<dyn Scene>>,
}

#[allow(dead_code)]
impl SceneManager {
    pub fn new<S>(scene: S) -> FerResult<Self>
    where S: Scene + 'static,
    {
        let mut manager = SceneManager {
            active_scene: "MAIN",
            scenes: HashMap::new(),
        };

        manager.add_scene("MAIN", scene)?;
        manager.set_active_scene("MAIN")?;

        Ok(manager)
    }

    pub fn add_scene<S>(&mut self, name: &'static str, mut scene: S) -> FerResult
    where
        S: Scene + 'static,
    {
        if self.scenes.contains_key(name) {
            return Err(FerError::SceneError("Scene Name Has Already Been Taken!"))
        }

        scene.load()?;
        self.scenes.insert(name, Box::new(scene));

        Ok(())
    }

    pub fn process(&mut self, ctx: core::GameContext) -> FerResult {
        let scene = self.scenes.get_mut(self.active_scene).unwrap().as_mut();

        scene.update(&ctx)?;
        scene.render(&ctx)?;

        Ok(())
    }

    fn remove_scene(&mut self, name: &str) -> FerResult {
        if !self.scenes.contains_key(name) || name == "MAIN" {
            return Err(FerError::SceneError("Cannot Delete Inexistent/MAIN Scene"))
        }

        let mut scene = self.scenes.remove(name).unwrap();
        scene.at_remove();

        Ok(())
    }

    fn remove_all(&mut self) {
        for (_, mut scene) in self.scenes.drain() {
            scene.at_remove();
        }
    }

    pub fn set_active_scene(&mut self, name: &'static str) -> FerResult<&'static str> {
        if !self.scenes.contains_key(name) {
            return Err(FerError::SceneError("Scene Doesn't Exist!"))
        }

        let last_scene = self.active_scene;
        self.active_scene = name;

        Ok(last_scene)
    }
}

impl Drop for SceneManager {
    fn drop(&mut self) {
        for (_, mut scene) in self.scenes.drain() {
            scene.at_remove();
        }
    }
}
