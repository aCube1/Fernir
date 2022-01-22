use ggez::input::keyboard::KeyCode;
use hecs::*;
use std::time::Duration;

use super::components::*;

pub fn apply_friction(world: &mut World) {
    let mut query = world.query::<(&mut Velocity, &Friction)>();

    for (_e, (vel, fric)) in query.iter() {
        let inverse_x = f32::signum(vel.x) * (-1.0);

        if f32::abs(vel.x) != 0.0 && f32::abs(vel.x) - fric.x > 0.0 {
            vel.x += fric.x * inverse_x;
        } else {
            vel.x = 0.0;
        }
    }
}

pub fn apply_velocity(world: &mut World) {
    let mut query = world.query::<(&mut Direction, &mut Velocity, &Acceleration, &MaxSpeed)>();

    for (_e, (dir, vel, accel, max_spd)) in query.iter() {
        if vel.x.abs() < max_spd.x {
            if f32::abs(vel.x + accel.x * dir.x) < max_spd.x {
                vel.x += accel.x * dir.x;
            } else {
                vel.x = max_spd.x * dir.x;
            }
        }

        println!("Player | Velocity -> X: {} | Y: {}", vel.x, vel.y);
        dir.x = 0.0;
    }
}

pub fn movement(world: &mut World, dt: &Duration) {
    let mut query = world.query::<(&mut Position, &Velocity)>();

    for (_e, (pos, vel)) in query.iter() {
        pos.x += vel.x * dt.as_secs_f32();
    }
}

pub fn input(world: &mut World, keycode: &KeyCode) {
    let mut query = world.query::<(&IsPlayer, &mut Direction)>();

    for (_e, (_, dir)) in query.iter() {
        match keycode {
            KeyCode::A => dir.x -= 1.0,
            KeyCode::D => dir.x += 1.0,
            _ => {}
        }
    }
}
