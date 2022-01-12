use image::ImageError;
use sdl2::{
    render::TextureValueError,
    video::WindowBuildError,
    IntegerOrSdlError
};
use std::fmt::{self, Display, Formatter};

pub type FerResult<T = ()> = Result<T, FerError>;

#[derive(Debug)]
pub enum FerError {
    /* SDL Errors */
    SdlInitError(String),
    SdlVideoError(String),
    SdlCanvasError(String),
    SdlWindowError(WindowBuildError),
    SdlRenderError(IntegerOrSdlError),
    SdlTextureError(TextureValueError),

    /* OpenGL Errors */
    GlContextError(String),

    /* Image Errors */
    ImageError(ImageError),

    /* Core Internal Errors */
    SceneError(&'static str),
    GraphicsError(&'static str),
}

impl Display for FerError {
    fn fmt(&self, format: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FerError::SdlInitError(msg) => writeln!(format, "[SDL_Init ERROR]: {}", msg),
            FerError::SdlVideoError(msg) => writeln!(format, "[SDL_Video ERROR]: {}", msg),
            FerError::SdlCanvasError(msg) => writeln!(format, "[SDL_Canvas ERROR]: {}", msg),
            FerError::SdlWindowError(error) => match error {
                WindowBuildError::SdlError(msg) => writeln!(format, "[SDL_Window ERROR]: {}", msg),
                WindowBuildError::InvalidTitle(title) => {
                    writeln!(format, "[SDL_Window ERROR]: Invalid Title {}", title)
                }
                WindowBuildError::WidthOverflows(width) => {
                    writeln!(format, "[SDL_Window ERROR]: Width Overflow ({})", width)
                }
                WindowBuildError::HeightOverflows(height) => {
                    writeln!(format, "[SDL_Window ERROR]: Height Overflow ({})", height)
                }
            },
            FerError::SdlRenderError(error) => match error {
                IntegerOrSdlError::SdlError(msg) => writeln!(format, "[SDL_Render ERROR]: {}", msg),
                IntegerOrSdlError::IntegerOverflows(msg, code) => writeln!(
                    format,
                    "[SDL_Render Integer Overflow Error]: {} | {}",
                    code, msg
                ),
            },
            FerError::SdlTextureError(error) => match error {
                TextureValueError::HeightOverflows(..) | TextureValueError::WidthOverflows(..) => {
                        writeln!(format, "[SDL_Texture Dimension Overflow ERROR]")
                }
                TextureValueError::SdlError(msg) => writeln!(format, "[SDL_Texture ERROR]: {}", msg),
                _ => writeln!(format, "[SDL_Texture Unknown Error]"),
            }

            FerError::GlContextError(msg) => writeln!(format, "[GL_Context ERROR]: {}", msg),

            FerError::ImageError(error) => match error {
                ImageError::Decoding(decoding_error) => {
                    writeln!(format, "[Image_Decoding ERROR]: {}", decoding_error)
                }
                ImageError::Limits(limit_error) => {
                    writeln!(format, "[Image_Limits ERROR]: {}", limit_error)
                }
                ImageError::Unsupported(unsupported_error) => {
                    writeln!(format, "[Image_Unsupported ERROR]: {}", unsupported_error)
                }
                _ => writeln!(format, "[Image ERROR]: Unknown"),
            },

            FerError::SceneError(msg) => writeln!(format, "[Scene Manager ERROR]: {}", msg),
            FerError::GraphicsError(msg) => writeln!(format, "[Graphics Manager ERROR]: {}", msg),
        }
    }
}
