use crate::core::{
    self,
    error::{FerError, FerResult},
};
use std::collections::HashMap;

pub trait Scene {
    fn load(&mut self) -> FerResult<Transition>;
    fn update(&mut self, _ctx: &mut core::GameContext) -> FerResult<Transition>;
    fn render(&mut self, _ctx: &mut core::GameContext) -> FerResult;

    fn at_remove(&mut self) {}
}

pub struct SceneManager {
    active_scene: &'static str,
    scenes: HashMap<&'static str, Box<dyn Scene>>,
}

#[allow(dead_code)]
pub enum Transition {
    None,
    Quit, /* Quit Current Scene and Delete All Scenes. */
    Push(&'static str, Box<dyn Scene>),
    Switch(&'static str),
    SwitchDelete(&'static str), /* Delete Current Scene and Switch to Another. */
}

impl SceneManager {
    pub fn new<S>(scene: S) -> FerResult<Self>
    where
        S: Scene + 'static,
    {
        let mut manager = SceneManager {
            active_scene: "MAIN",
            scenes: HashMap::new(),
        };

        manager.add_scene("MAIN", Box::new(scene))?;
        manager.set_active_scene("MAIN")?;

        Ok(manager)
    }

    pub fn process(&mut self, mut ctx: core::GameContext) -> FerResult {
        let scene = self.scenes.get_mut(self.active_scene).unwrap().as_mut();

        let transition = scene.update(&mut ctx)?;
        scene.render(&mut ctx)?;

        self.process_transition(transition)?;
        Ok(())
    }

    fn process_transition(&mut self, transition: Transition) -> FerResult {
        match transition {
            Transition::None => {}
            Transition::Quit => self.remove_all(),
            Transition::Push(name, new_scene) => self.add_scene(name, new_scene)?,
            Transition::Switch(name) => {
                match self.set_active_scene(name) {
                    Ok(last_scene) => println!("{} Scene Switched to {}", last_scene, name), /* TODO: Use a Better Logger */
                    Err(error) => return Err(error),
                }
            }
            Transition::SwitchDelete(name) => {
                self.remove_scene(self.active_scene)?;
                match self.set_active_scene(name) {
                    Ok(last_scene) => println!("{} Scene Switched to {}", last_scene, name), /* TODO: Use a Better Logger*/
                    Err(error) => return Err(error),
                }
            }
        }

        Ok(())
    }

    fn add_scene(&mut self, name: &'static str, mut scene: Box<dyn Scene>) -> FerResult {
        if self.scenes.contains_key(name) {
            return Err(FerError::SceneError("Scene Name Has Already Been Taken!"));
        }

        let transition = scene.load()?;
        self.scenes.insert(name, scene);

        self.process_transition(transition)?;
        Ok(())
    }

    fn remove_scene(&mut self, name: &str) -> FerResult {
        if !self.scenes.contains_key(name) || name == "MAIN" {
            return Err(FerError::SceneError("Cannot Delete Inexistent/MAIN Scene"));
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
            return Err(FerError::SceneError("Scene Doesn't Exist!"));
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
