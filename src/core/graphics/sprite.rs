use sdl2::{
    render::Texture,
    surface::Surface,
    rect::{Point, Rect},
    pixels::PixelFormatEnum
};
use image::{self, ColorType};
use crate::core::error::*;
use super::Graphics;

pub struct SpriteId(&'static str);

pub struct Sprite {
    texture: Texture,
    src_rect: Rect,
    scale: Point,
    center: Point,
    angle: f64,
    flip_horizontal: bool,
    flip_vertical: bool,
}

impl Graphics {
    pub fn new_sprite(&mut self, path: &'static str) -> FerResult<SpriteId> {
        let img = image::open(path).map_err(FerError::ImageError)?;
        let mut data = img.to_bytes();
        let (width, height): (u32, u32);
        let (pixel_format, pitch) = match img.color() {
            ColorType::Rgb8 => {
                let image = img.as_rgb8().unwrap();
                width = image.width();
                height = image.height();
                (PixelFormatEnum::RGB24, width * 3)
            }
            ColorType::Rgba8 => {
                let image = img.as_rgba8().unwrap();
                width = image.width();
                height = image.height();
                (PixelFormatEnum::RGBA32, width * 4)
            }
            _ => return Err(FerError::GraphicsError("Cannot Identify PixelFormat of Texture!")),
        };
        let surface = Surface::from_data(data.as_mut(), width, height, pitch, pixel_format)
            .map_err(FerError::SdlCanvasError)?;

        let texture = surface.as_texture(&self.texture_creator)
            .map_err(FerError::SdlTextureError)?;

        let sprite = Sprite {
            texture,
            src_rect: Rect::new(0, 0, width, height),
            scale: Point::new(1, 1),
            center: Point::new((width / 2) as i32, (height / 2) as i32),
            angle: 0.0,
            flip_horizontal: false,
            flip_vertical: false,
        };

        self.loaded_sprites.insert(path, sprite);
        Ok(SpriteId(path))
    }

    pub fn render_sprite(&mut self, sprite_id: &SpriteId, x: i32, y: i32, width: u32, height: u32) -> FerResult {
        if !self.loaded_sprites.contains_key(sprite_id.0) {
            return Err(FerError::GraphicsError("Sprite ID Doesn't Exist!"));
        }

        let sprite = self.loaded_sprites.get(sprite_id.0).unwrap();
        let dst_rect = Rect::new(x, y, width * (sprite.scale.x as u32), height * (sprite.scale.y as u32));

        self.canvas.copy_ex(&sprite.texture, sprite.src_rect, dst_rect, sprite.angle, sprite.center, sprite.flip_horizontal, sprite.flip_vertical)
            .map_err(FerError::SdlCanvasError)?;

        Ok(())
    }

    pub fn destroy_sprite(&mut self, sprite_id: SpriteId) -> FerResult {
        if !self.loaded_sprites.contains_key(sprite_id.0) {
            return Err(FerError::GraphicsError("Sprite ID Doesn't Exist!"));
        }

        let sprite = self.loaded_sprites.remove(sprite_id.0).expect("Cannot Remove Sprite!");
        unsafe {
            sprite.texture.destroy();
        }

        Ok(())
    }
}

#[allow(dead_code)]
impl Sprite {
    #[inline]
    pub fn set_horizontal_flip(&mut self, active: bool) {
        self.flip_horizontal = active;
    }
    #[inline]
    pub fn set_vertical_flip(&mut self, active: bool) {
        self.flip_vertical = active;
    }
}
