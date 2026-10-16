use bevy::{
    animation::{
        AnimatedBy, AnimationClip, AnimationPlayer,
        graph::{AnimationGraph, AnimationGraphHandle},
    },
    app::{App, Plugin, PreUpdate},
    asset::Assets,
    ecs::{
        lifecycle::Add,
        observer::On,
        system::{Commands, Populated, Query, ResMut},
    },
    time::DelayedCommandsExt,
    transform::components::Transform,
    ui::{UiTransform, Val2},
};

use crate::animate::{AnimatableUiTranslation, Animate, InitialAnimateValue};

/// Plugin for handling animations using [Animate]
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, sync_animatable_ui_translation)
            .add_observer(on_add_animate);
    }
}

fn sync_animatable_ui_translation(
    mut query: Populated<(&mut UiTransform, &AnimatableUiTranslation)>,
) {
    query
        .par_iter_mut()
        .for_each(|(mut transform, AnimatableUiTranslation(translation))| {
            transform.translation = Val2::px(translation.x, translation.y)
        });
}

fn on_add_animate(
    trigger: On<Add<Animate>>,
    mut commands: Commands,
    animates: Query<(&Animate, Option<&Transform>, Option<&UiTransform>)>,
    mut animations: ResMut<Assets<AnimationClip>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let entity = trigger.entity;

    if let Ok((animate, transform_option, ui_transform_option)) = animates.get(entity) {
        let animation_handle = animations.add(animate.animation.clone());

        let (graph, node_index) = AnimationGraph::from_clip(animation_handle);
        let graph_handle = graphs.add(graph);

        let mut player = AnimationPlayer::default();

        player.play(node_index);

        let transform = transform_option.cloned().unwrap_or_default();
        let mut ui_transform = ui_transform_option.cloned().unwrap_or_default();

        if let Some(ref init) = animate.init {
            if animate.delay == 0.0 {
                match init {
                    InitialAnimateValue::Translate(start) => {
                        commands
                            .entity(entity)
                            .insert(transform.with_translation(*start));
                    }
                    InitialAnimateValue::Rotate(start) => {
                        commands
                            .entity(entity)
                            .insert(transform.with_rotation(*start));
                    }
                    InitialAnimateValue::Scale(start) => {
                        commands.entity(entity).insert(transform.with_scale(*start));
                    }
                    // Handled below
                    InitialAnimateValue::UiTranslate(..) => {}
                    InitialAnimateValue::UiRotate(start) => {
                        ui_transform.rotation = *start;
                        commands.entity(entity).insert(ui_transform);
                    }
                    InitialAnimateValue::UiScale(start) => {
                        ui_transform.scale = *start;
                        commands.entity(entity).insert(ui_transform);
                    }
                }
            }

            // This struct must always be added to make ui translation work
            if let InitialAnimateValue::UiTranslate(start) = init {
                commands
                    .entity(entity)
                    .insert(AnimatableUiTranslation(*start));
            }
        }

        commands
            .delayed()
            .secs(animate.delay)
            .entity(entity)
            // TODO: repeatable animations?
            .remove::<Animate>()
            .insert((
                player,
                AnimationGraphHandle(graph_handle),
                animate.target_id,
                AnimatedBy(entity),
            ));
    }
}
