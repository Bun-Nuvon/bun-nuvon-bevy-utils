use bevy::{
    app::{App, Plugin, PostUpdate},
    color::Color,
    ecs::{
        entity::Entity,
        hierarchy::Children,
        observer::On,
        query::{Has, With, Without},
        schedule::IntoScheduleConfigs,
        system::{Commands, Populated, Query, Res},
    },
    input_focus::{InputFocus, tab_navigation::TabNavigationPlugin},
    picking::{cursor::CursorIconPlugin, hover::Hovered},
    ui::{ComputedNode, Node, Outline, UiSystems, Val},
    ui_widgets::{SliderRange, SliderThumb, SliderValue, ValueChange},
};

use crate::{Button, Progress, ProgressFill, Slider};

/// Plugin for all widget systems
pub struct UiPlugin {
    /// The outline color of focuses widgets
    pub focus_color: Color,
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((TabNavigationPlugin, CursorIconPlugin))
            .add_systems(
                PostUpdate,
                (
                    focus_outline(self.focus_color).in_set(UiSystems::Content),
                    slider_change,
                    update_progress,
                ),
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

fn slider_change(
    sliders: Query<(Entity, &ComputedNode, &SliderValue, &SliderRange), With<Slider>>,
    children: Query<&Children>,
    mut thumbs: Query<(&mut Node, &ComputedNode, Has<SliderThumb>), Without<Slider>>,
) {
    for (entity, parent_computed, value, range) in sliders {
        for child in children.iter_descendants(entity) {
            if let Ok((mut node, computed, is_thumb)) = thumbs.get_mut(child)
                && is_thumb
            {
                node.left = Val::Px(
                    range.thumb_position(value.0)
                        * (parent_computed.size().x - computed.size().x)
                        * parent_computed.inverse_scale_factor(),
                );
            }
        }
    }
}

fn update_progress(
    progresses: Query<(Entity, &Progress)>,
    children: Query<&Children>,
    mut fills: Query<(&mut Node, Has<ProgressFill>), Without<Progress>>,
) {
    for (entity, progress) in progresses {
        for child in children.iter_descendants(entity) {
            if let Ok((mut node, is_inner)) = fills.get_mut(child)
                && is_inner
            {
                node.width = Val::Percent(progress.0 * 100.0);
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
