use std::time::Duration;

use bevy::{
    app::{App, Plugin, PostUpdate, PreUpdate, Update},
    camera::{Camera, NormalizedRenderTarget},
    color::Color,
    ecs::{
        entity::Entity,
        hierarchy::Children,
        observer::On,
        query::{Changed, Has, With, Without},
        schedule::IntoScheduleConfigs,
        system::{Commands, Populated, Query, Res, Single},
    },
    input::{ButtonInput, keyboard::KeyCode},
    input_focus::{
        InputFocus,
        tab_navigation::{TabIndex, TabNavigationPlugin},
    },
    math::Vec2,
    picking::{
        backend::HitData,
        cursor::{CursorIconPlugin, EntityCursor},
        events::{Pointer, PointerClick},
        hover::Hovered,
        pointer::{Location, PointerButton, PointerId},
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
    pub focus_color: Color,
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((TabNavigationPlugin, CursorIconPlugin))
            .add_systems(PreUpdate, click_focus)
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

/// Click button when focused and keys are pressed
fn click_focus(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    focus: Res<InputFocus>,
    buttons: Query<(), With<Button>>,
    camera: Single<Entity, With<Camera>>,
) {
    if keys.any_just_pressed(vec![KeyCode::Enter, KeyCode::NumpadEnter, KeyCode::Space])
        && let Some(entity) = focus.get()
        && buttons.contains(entity)
    {
        // Is there a better way to do this?
        commands.trigger(PointerClick {
            entity,
            pointer: Pointer::new(
                PointerId::Mouse,
                Location {
                    target: NormalizedRenderTarget::None {
                        width: 1,
                        height: 1,
                    },
                    position: Vec2::ZERO,
                },
            ),
            button: PointerButton::Primary,
            hit: HitData::new(camera.into_inner(), 0.0, None, None),
            duration: Duration::from_millis(1),
            count: 1,
        });
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
