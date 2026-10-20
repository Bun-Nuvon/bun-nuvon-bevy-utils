use bevy::{
    ecs::component::Component,
    input_focus::tab_navigation::TabIndex,
    picking::{cursor::EntityCursor, hover::Hovered},
    window::SystemCursorIcon,
};
use proc_macros::marker_component_bundle;

/// Helper component for a button widget
#[marker_component_bundle]
#[require(
    bevy::ui_widgets::Button,
    Hovered,
    TabIndex,
    EntityCursor::System(SystemCursorIcon::Pointer)
)]
pub struct Button;

/// Helper component for a slider widget
///
/// # Usage
/// ```rust,no_run
/// bsn! {
///     Node {
///         align_items: AlignItems::Stretch,
///         column_gap: Val::Px(4.0),
///         display: Display::Flex,
///         flex_direction: FlexDirection::Column,
///         height: Val::Px(24.0),
///         justify_content: JustifyContent::Center,
///         justify_items: JustifyItems::Center,
///         width: Val::Percent(100.0),
///     }
///     Slider
///     SliderValue(initial)
///     SliderRange::new(start, end)
///     Children [
///         // Background rail
///         Node {
///             border_radius: BorderRadius::MAX,
///             height: Val::Px(12.0),
///         }
///         --
///         Node {
///             bottom: Val::ZERO,
///             display: Display::Flex,
///             left: Val::ZERO,
///             position_type: PositionType::Absolute,
///             // Track is short by 24px to accommodate the thumb
///             right: Val::Px(24.0),
///             top: Val::ZERO,
///         }
///         Children [
///             Node {
///                 border_radius: BorderRadius::MAX,
///                 display: Display::Flex,
///                 height: Val::Px(24.0),
///                 left: Val::ZERO,
///                 position_type: PositionType::Absolute,
///                 width: Val::Px(24.0),
///             }
///             SliderThumb
///         ]
///     ]
/// }
/// ```
#[marker_component_bundle]
#[require(
    bevy::ui_widgets::Slider,
    Hovered,
    TabIndex,
    EntityCursor::System(SystemCursorIcon::EwResize)
)]
pub struct Slider;

/// Helper component for a progress widget
#[derive(Clone, Component, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Progress(pub f32);

/// Helper component for the "progress" of the progress widget
#[marker_component_bundle]
pub struct ProgressInner;
