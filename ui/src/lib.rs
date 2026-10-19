#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use bevy::{
    app::{App, Plugin, PostUpdate, Update},
    color::Color,
    ecs::{
        entity::Entity,
        hierarchy::Children,
        observer::On,
        query::{Changed, Has, With, Without},
        schedule::IntoScheduleConfigs,
        system::{Commands, Populated, Query, Res},
    },
    input_focus::{
        InputFocus,
        tab_navigation::{TabIndex, TabNavigationPlugin},
    },
    picking::{
        cursor::{CursorIconPlugin, EntityCursor},
        hover::Hovered,
    },
    ui::{Node, Outline, UiSystems, Val},
    ui_widgets::{SliderRange, SliderThumb, SliderValue, ValueChange},
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
#[marker_component_bundle]
#[require(
    bevy::ui_widgets::Slider,
    Hovered,
    TabIndex,
    EntityCursor::System(SystemCursorIcon::EwResize)
)]
pub struct Slider;

/// Plugin for all widget systems
pub struct UiPlugin {
    /// The outline color of focuses widgets
    pub focus_color: Color,
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((TabNavigationPlugin, CursorIconPlugin))
            .add_systems(Update, slider_change)
            .add_systems(
                PostUpdate,
                focus_outline(self.focus_color).in_set(UiSystems::Content),
            )
            .add_observer(on_slider_change);
    }
}

fn focus_outline(
    focus_color: Color,
) -> impl Fn(Commands, Populated<(Entity, &Hovered), With<Button>>, Res<InputFocus>) {
    move |mut commands: Commands,
          elements: Populated<(Entity, &Hovered), With<Button>>,
          focus: Res<InputFocus>| {
        for (entity, Hovered(hovered)) in elements {
            if focus.get() == Some(entity) || *hovered {
                commands.entity(entity).insert(Outline {
                    color: focus_color,
                    width: Val::Px(2.0),
                    offset: Val::Px(2.0),
                });
            } else {
                commands.entity(entity).remove::<Outline>();
            }
        }
    }
}

fn on_slider_change(
    trigger: On<ValueChange<f32>>,
    mut commands: Commands,
    sliders: Populated<(), With<Slider>>,
) {
    if sliders.contains(trigger.source) {
        commands
            .entity(trigger.source)
            .insert(SliderValue(trigger.value));
    }
}

fn slider_change(
    sliders: Query<(Entity, &SliderValue, &SliderRange), (Changed<SliderValue>, With<Slider>)>,
    children: Query<&Children>,
    mut thumbs: Query<(&mut Node, Has<SliderThumb>), Without<Slider>>,
) {
    for (entity, value, range) in sliders {
        for child in children.iter_descendants(entity) {
            if let Ok((mut node, is_thumb)) = thumbs.get_mut(child)
                && is_thumb
            {
                node.left = Val::Percent(range.thumb_position(value.0) * 100.0);
            }
        }
    }
}
