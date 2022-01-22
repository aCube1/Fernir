use mint::*;
/* Tags */
pub struct IsPlayer;

/* Components */
#[derive(Copy, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Into<Point2<f32>> for Position {
    fn into(self) -> Point2<f32> {
        Point2 { x: self.x, y: self.y }
    }
}

pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

pub struct MaxSpeed {
    pub x: f32,
    pub y: f32,
}

pub struct Acceleration {
    pub x: f32,
    pub y: f32,
}

pub struct Friction {
    pub x: f32,
    pub y: f32,
}

pub struct Direction {
    pub x: f32,
    pub y: f32,
}
