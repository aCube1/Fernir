use sdl2::{
    VideoSubsystem,
    render::{WindowCanvas, TextureCreator},
    video::{Window, WindowContext, GLContext},
};
use super::error::*;

pub struct Graphics {
    gl_context: GLContext,
    canvas: WindowCanvas,
    texture_creator: TextureCreator<WindowContext>
}

impl Graphics {
    pub fn new(video: &VideoSubsystem, window: Window) -> FerResult<Self> {
        gl::load_with(|s| video.gl_get_proc_address(s) as *const std::ffi::c_void);
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);


        let gl_context = window.gl_create_context().map_err(FerError::GlContextError)?;
        let canvas = window.into_canvas()
            .present_vsync()
            .accelerated()
            .target_texture()
            .build()
            .map_err(FerError::SdlRenderError)?;

        let texture_creator = canvas.texture_creator();

        println!("OpenGL Version: {}.{}", gl::MAJOR_VERSION, gl::MINOR_VERSION);
        Ok(Graphics {
            gl_context,
            canvas,
            texture_creator,
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
