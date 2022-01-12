use std::collections::HashMap;
use super::error::*;
use sdl2::{
    render::{TextureCreator, WindowCanvas},
    video::{GLContext, Window, WindowContext},
    VideoSubsystem,
};

mod sprite;
pub use sprite::Sprite;
pub use sprite::SpriteId;

#[allow(dead_code)]
pub struct Graphics {
    gl_context: GLContext,
    canvas: WindowCanvas,
    texture_creator: TextureCreator<WindowContext>,
    loaded_sprites: HashMap<&'static str, Sprite>,
}

#[allow(dead_code)]
impl Graphics {
    pub fn new(video: &VideoSubsystem, window: Window) -> FerResult<Self> {
        gl::load_with(|s| video.gl_get_proc_address(s) as *const std::ffi::c_void);
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        let gl_context = window
            .gl_create_context()
            .map_err(FerError::GlContextError)?;
        let canvas = window
            .into_canvas()
            .present_vsync()
            .accelerated()
            .target_texture()
            .build()
            .map_err(FerError::SdlRenderError)?;

        let texture_creator = canvas.texture_creator();

        Ok(Graphics {
            gl_context,
            canvas,
            texture_creator,
            loaded_sprites: HashMap::new(),
        })
    }

    #[inline]
    pub fn clear(&mut self) {
        self.canvas.clear();
    }
    #[inline]
    pub fn present(&mut self) {
        self.canvas.present();
    }
}
