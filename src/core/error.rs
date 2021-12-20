use std::fmt::{self, Display, Formatter};
use sdl2::{IntegerOrSdlError, video::WindowBuildError};

pub type FerResult<T = ()> = Result<T, FerError>;

#[derive(Debug)]
pub enum FerError {
    /* SDL Errors */
    SdlInitError(String),
    SdlTimerError(String),
    SdlVideoError(String),
    SdlWindowError(WindowBuildError),
    SdlRenderError(IntegerOrSdlError),

    /* Core Internal Errors */
    SceneError(&'static str),
}

impl Display for FerError {
    fn fmt(&self, format: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FerError::SdlInitError(msg) => writeln!(format, "[SDL_Init Error]: {}", msg),
            FerError::SdlTimerError(msg) => writeln!(format, "[SDL_Timer Error]: {}", msg),
            FerError::SdlVideoError(msg) => writeln!(format, "[SDL_Video Error]: {}", msg),
            FerError::SdlWindowError(error) => {
                match error {
                    WindowBuildError::SdlError(msg) => writeln!(format, "[SDL_Window Error]: {}", msg),
                    WindowBuildError::InvalidTitle(title) => writeln!(format, "[SDL_Window Error]: Invalid Title {}", title),
                    WindowBuildError::WidthOverflows(width) => writeln!(format, "[SDL_Window Error]: Width Overflow ({})", width),
                    WindowBuildError::HeightOverflows(height) => writeln!(format, "[SDL_Window Error]: Height Overflow ({})", height),
                }
            }
            FerError::SdlRenderError(error) => {
                match error {
                    IntegerOrSdlError::SdlError(msg) => writeln!(format, "[SDL_Render Error]: {}", msg),
                    IntegerOrSdlError::IntegerOverflows(msg, code) => writeln!(format, "[SDL_Render Integer Overflow Error]: {} | {}", code, msg),
                }
            }

            FerError::SceneError(msg) => write!(format, "[Scene Manager ERROR]: {}", msg),
        }
    }
}
