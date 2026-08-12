pub use proc_macros::*;

use bevy::math::Vec2;

pub fn snap_to_grid(pos: impl Into<Vec2>, size: f32) -> Vec2 {
    (pos.into() / size).round() * size
}
