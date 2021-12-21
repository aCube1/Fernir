use std::time::Instant;

pub struct Timer {
    timer: Instant,

    fps: u32,
    dt: f64,
    fps_timer: f32,
    fps_count: u32,
    last_frame: u128,
}

impl Timer {
    pub fn new() -> Self {
        let timer = Instant::now();

        Timer {
            timer,
            fps: 0,
            dt: 0.0,
            fps_timer: 0.0,
            last_frame: 0,
            fps_count: 0,
        }
    }

    pub fn update(&mut self) {
        self.dt = (self.timer.elapsed().as_millis() - self.last_frame) as f64 / 1000.0;
        self.fps_timer += self.dt as f32;
        if self.fps_timer >= 1.0 {
            self.fps = self.fps_count;
            self.fps_count = 0;
            self.fps_timer = 0.0;
        }

        self.fps_count += 1;
        self.last_frame = self.timer.elapsed().as_millis();
    }

    #[inline]
    pub fn get_fps(&self) -> u32 {
        self.fps
    }

    #[inline]
    pub fn get_dt(&self) -> f64 {
        self.dt
    }
}
