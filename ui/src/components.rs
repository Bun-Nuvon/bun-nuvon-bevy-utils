use bevy::{
    ecs::component::Component,
    input_focus::tab_navigation::TabIndex,
    picking::{cursor::EntityCursor, hover::Hovered},
    ui::Node,
    window::SystemCursorIcon,
};
use proc_macros::marker_component_bundle;

/// Helper component for a button widget. Use [button](crate::button) to create the widget
#[marker_component_bundle]
#[require(
    bevy::ui_widgets::Button,
    Hovered,
    TabIndex,
    EntityCursor::System(SystemCursorIcon::Pointer)
)]
pub struct Button;

/// Helper component for a slider widget. Use [slider](crate::slider) to create the widget
#[marker_component_bundle]
#[require(
    bevy::ui_widgets::Slider,
    Hovered,
    TabIndex,
    EntityCursor::System(SystemCursorIcon::EwResize)
)]
pub struct Slider;

/// Helper component for a progress widget. Use [progress](crate::progress) to create the widget
#[derive(Clone, Component, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Progress(pub f32);

/// Helper component for the "progress" of the progress widget
#[marker_component_bundle]
// Require the Node component if users want to make a custom fill
#[require(Node)]
pub struct ProgressFill;
