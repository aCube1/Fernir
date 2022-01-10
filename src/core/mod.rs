use sdl2::{
    Sdl,
    event::Event,
    VideoSubsystem
};
use gl;

pub mod error;
pub mod scene;
pub mod timer;
pub mod graphics;

pub use error::*;

#[allow(dead_code)]
pub struct Core {
    sdl_ctx: Sdl,
    video: VideoSubsystem,

    graphics: graphics::Graphics,
    timer: timer::Timer,
    scene_manager: scene::SceneManager,

    running: bool,
}

pub struct GameContext<'a> {
    pub timer: &'a timer::Timer,
}

pub struct CoreBuilder {
    title: String,
    width: u32,
    height: u32,
}

impl Default for CoreBuilder {
    fn default() -> Self {
        Self {
            title: "Fernir".into(),
            width: 800,
            height: 600,
        }
    }
}

#[allow(dead_code)]
impl CoreBuilder {
    #[inline]
    pub fn new() -> Self {
        CoreBuilder::default()
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn with_height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    pub fn with_size(self, width: u32, height: u32) -> Self {
        self.with_width(width).with_height(height)
    }

    pub fn build<S>(self, main_scene: S)-> FerResult<Core>
    where S: scene::Scene + 'static {
        let sdl_ctx = sdl2::init().map_err(FerError::SdlInitError)?;
        let video = sdl_ctx.video().map_err(FerError::SdlVideoError)?;

        gl::load_with(|s| video.gl_get_proc_address(s) as *const std::ffi::c_void);

        let window = video
            .window(&self.title[..], self.width, self.height)
            .position_centered()
            .opengl()
            .build()
            .map_err(FerError::SdlWindowError)?;

        let graphics = graphics::Graphics::new(&video, window)?;
        let timer = timer::Timer::new();
        let scene_manager = scene::SceneManager::new(main_scene)?;

        Ok(Core {
            sdl_ctx,
            video,
            graphics,
            timer,
            scene_manager,
            running: true,
        })
    }
}

impl Core {
    pub fn run(mut self) -> FerResult {
        let mut event_pump = self.sdl_ctx.event_pump().unwrap();

        while self.running {
            /* NOTE: PlaceHolder */
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => self.running = false,
                    _ => (),
                }
            }

            self.timer.update();

            self.graphics.clear();
            self.scene_manager.process(GameContext {
                timer: &self.timer,
            })?;
            self.graphics.present();
        }

        Ok(())
    }
}
