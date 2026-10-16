use bevy::{
    animation::{
        AnimationClip, AnimationTargetId, animated_field,
        animation_curves::{AnimatableCurve, AnimatableKeyframeCurve, AnimatableProperty},
    },
    asset::uuid::Uuid,
    curve::{Curve, CurveExt, EaseFunction, EasingCurve, interval},
    ecs::{component::Component, reflect::ReflectComponent},
    math::{Quat, Rot2, Vec2, Vec3},
    reflect::{FromReflect, GetTypeRegistration, Reflect, Typed, std_traits::ReflectDefault},
    transform::components::Transform,
    ui::UiTransform,
    utils::default,
};

#[derive(Clone, Debug)]
pub(crate) enum InitialAnimateValue {
    Translate(Vec3),
    UiTranslate(Vec2),
    Rotate(Quat),
    UiRotate(Rot2),
    Scale(Vec3),
    UiScale(Vec2),
}

#[derive(Clone, Component, Debug)]
pub struct Animate {
    pub animation: AnimationClip,
    pub target_id: AnimationTargetId,
    pub delay: f32,
    pub(crate) init: Option<InitialAnimateValue>,
}

impl Default for Animate {
    fn default() -> Self {
        Self {
            target_id: AnimationTargetId(Uuid::default()),
            ..default()
        }
    }
}

impl Animate {
    fn build<P>(
        init: impl Into<Option<InitialAnimateValue>>,
        property: P,
        start: P::Property,
        end: P::Property,
        time: f32,
        ease_function: EaseFunction,
    ) -> Self
    where
        P: AnimatableProperty + Clone,
        P::Property: Clone + std::fmt::Debug + FromReflect + GetTypeRegistration + Typed,
        AnimatableKeyframeCurve<P::Property>: Curve<P::Property>,
        EasingCurve<<P as AnimatableProperty>::Property>:
            Curve<<P as AnimatableProperty>::Property>,
    {
        let mut animation = AnimationClip::default();
        let target_id = AnimationTargetId(Uuid::new_v4());

        animation.add_curve_to_target(
            target_id,
            AnimatableCurve::new(
                property,
                EasingCurve::new(start, end, ease_function)
                    .reparametrize_linear(
                        interval(0.0, time).expect("time should always be greater than 0"),
                    )
                    .expect("this curve has bounded domain, so this should never fail"),
            ),
        );

        Self {
            animation,
            target_id,
            delay: 0.0,
            init: init.into(),
        }
    }

    pub fn translate(start: Vec3, end: Vec3, time: f32, ease_function: EaseFunction) -> Self {
        Self::build(
            InitialAnimateValue::Translate(start),
            animated_field!(Transform::translation),
            start,
            end,
            time,
            ease_function,
        )
    }

    pub fn ui_translate(start: Vec2, end: Vec2, time: f32, ease_function: EaseFunction) -> Self {
        Self::build(
            InitialAnimateValue::UiTranslate(start),
            animated_field!(AnimatableUiTranslation::0),
            start,
            end,
            time,
            ease_function,
        )
    }

    pub fn rotate(start: Quat, end: Quat, time: f32, ease_function: EaseFunction) -> Self {
        Self::build(
            InitialAnimateValue::Rotate(start),
            animated_field!(Transform::rotation),
            start,
            end,
            time,
            ease_function,
        )
    }

    pub fn ui_rotate(start: Rot2, end: Rot2, time: f32, ease_function: EaseFunction) -> Self {
        Self::build(
            InitialAnimateValue::UiRotate(start),
            animated_field!(UiTransform::rotation),
            start,
            end,
            time,
            ease_function,
        )
    }

    pub fn scale(start: Vec3, end: Vec3, time: f32, ease_function: EaseFunction) -> Self {
        Self::build(
            InitialAnimateValue::Scale(start),
            animated_field!(Transform::scale),
            start,
            end,
            time,
            ease_function,
        )
    }

    pub fn ui_scale(start: Vec2, end: Vec2, time: f32, ease_function: EaseFunction) -> Self {
        Self::build(
            InitialAnimateValue::UiScale(start),
            animated_field!(UiTransform::scale),
            start,
            end,
            time,
            ease_function,
        )
    }

    pub fn with_delay(mut self, delay: f32) -> Self {
        self.delay = delay;
        self
    }
}

#[derive(Clone, Component, Copy, Debug, Default, PartialEq, Reflect)]
#[reflect(Clone, Component, Debug, Default, PartialEq)]
pub struct AnimatableUiTranslation(pub Vec2);
