use sdl2::TimerSubsystem;
use crate::core::error::*;

pub struct Timer {
    timer: TimerSubsystem,

    fps: u32,
    dt: f64,
    fps_timer: f32,
    last_frame: u32,
    fps_count: u32,
}

impl Timer {
    pub fn new(sdl_cxt: &sdl2::Sdl) -> FerResult<Self> {
        let timer = sdl_cxt.timer().map_err(FerError::SdlTimerError)?;

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
        self.dt = (self.timer.ticks() - self.last_frame) as f64 / 1000.0;
        self.fps_timer += self.dt as f32;
        if self.fps_timer >= 1.0 {
            self.fps = self.fps_count;
            self.fps_count = 0;
            self.fps_timer = 0.0;
        }

        self.fps_count += 1;
        self.last_frame = self.timer.ticks();
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
