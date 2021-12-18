use sdl2::{event::Event, render::WindowCanvas, Sdl, VideoSubsystem};

pub mod scene;
pub mod timer;

#[allow(dead_code)]
pub struct Core {
    sdl_ctx: Sdl,
    video: VideoSubsystem,
    canvas: WindowCanvas,

    timer: timer::Timer,
    scene_manager: scene::SceneManager,

    running: bool,
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

    pub fn build<S>(self, main_scene: S)-> Result<Core, String>
    where S: scene::Scene + 'static {
        let sdl_ctx = sdl2::init()?;
        let video = sdl_ctx.video()?;

        let window = video
            .window(&self.title[..], self.width, self.height)
            .position_centered()
            .opengl()
            .build()
            .expect("Couldn't Initialize Window");

        let canvas = window
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .expect("Couldn't Create Window Canvas");

        let timer = timer::Timer::new(&sdl_ctx)?;

        let mut scene_manager = scene::SceneManager::new();
        scene_manager.add_state("MAIN", main_scene)?;
        scene_manager.set_active_scene("MAIN")?;

        Ok(Core {
            sdl_ctx,
            video,
            canvas,
            timer,
            scene_manager,
            running: true,
        })
    }
}

impl Core {
    pub fn run(mut self) -> Result<(), String> {
        let mut event_pump = self.sdl_ctx.event_pump().unwrap();

        while self.running {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => self.running = false,
                    _ => (),
                }
            }

            self.timer.update();
            self.canvas.clear();
            self.canvas.present();
        }

        Ok(())
    }
}
