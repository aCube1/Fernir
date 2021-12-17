use sdl2::TimerSubsystem;

pub struct Timer {
    timer: TimerSubsystem,

    fps: u32,
    dt: f64,
    fps_timer: f64,
    last_frame: u64,
    fps_count: u32,
}

impl Timer {
    pub fn new(sdl_cxt: &sdl2::Sdl) -> Result<Self, String> {
        let timer = sdl_cxt.timer()?;

        Ok(Timer {
            timer,
            fps: 0,
            dt: 0.0,
            fps_timer: 0.0,
            last_frame: 0,
            fps_count: 0,
        })
    }

    pub fn update(&mut self) {
        self.last_frame = 0;
        self.fps_count = 0;

        self.dt = (self.timer.performance_counter() - self.last_frame) as f64 / 1000.0;
        self.fps_timer += self.dt;
        if self.fps_timer >= 1.0 {
            self.fps = self.fps_count;
            self.fps_count = 0;
            self.fps_timer = 0.0;
        }

        self.fps_count += 1;
        self.last_frame = self.timer.performance_counter();
    }
}
