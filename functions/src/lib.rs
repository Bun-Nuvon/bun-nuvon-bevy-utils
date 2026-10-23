#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use bevy::{
    math::Vec2,
    scene::{Scene, bsn},
};

/// Snap position to a grid
pub fn snap_to_grid(pos: impl Into<Vec2>, size: f32) -> Vec2 {
    (pos.into() / size).round() * size
}

/// Map data to a scene or its default based on a function call
pub fn scene_or<D, O>(data: D, scene: impl FnOnce(D) -> O, default: impl Scene) -> impl Scene
where
    O: Into<Option<Box<dyn Scene>>>,
{
    scene(data).into().unwrap_or(Box::new(default))
}
/// Map data to a scene or an empty scene based on a function call
pub fn scene_or_empty<D, O>(data: D, scene: impl FnOnce(D) -> O) -> impl Scene
where
    O: Into<Option<Box<dyn Scene>>>,
{
    scene_or(data, scene, bsn! {})
}
