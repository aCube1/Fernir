use hecs::World;

use super::components::*;

pub fn spawn_player(world: &mut World) {
    let _ = world.spawn((
        IsPlayer,
        Position { x: 400.0, y: 300.0 },
        MaxSpeed { x: 300.0, y: 0.0 },
        Acceleration { x: 50.0, y: 0.0 },
        Friction { x: 25.0, y: 0.0 },
        Velocity { x: 0.0, y: 0.0 },
        Direction { x: 0.0, y: 0.0 },
    ));
}
