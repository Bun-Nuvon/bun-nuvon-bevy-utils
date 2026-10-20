#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

pub use animate::*;
pub use plugin::AnimationPlugin;

mod animate;
mod plugin;
