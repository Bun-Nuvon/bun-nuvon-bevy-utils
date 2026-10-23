use bevy::{
    color::Color,
    ecs::{hierarchy::Children, template::template},
    scene::{Scene, bsn},
    text::{Justify, TextColor, TextLayout},
    ui::{
        AlignItems, BackgroundColor, BoxShadow, JustifyContent, Node, Overflow, PositionType,
        ShadowStyle, Val, widget::Text,
    },
    ui_widgets::{SliderRange, SliderThumb, SliderValue},
};
use functions::scene_or_empty;

use crate::{Button, ButtonConfig, Progress, ProgressConfig, ProgressFill, Slider, SliderConfig};

fn shadow(shadow: Option<Vec<ShadowStyle>>) -> impl Scene {
    scene_or_empty(shadow, |shadow| {
        shadow.map(|styles| {
            Box::new(bsn! {
                BoxShadow(styles)
            }) as Box<dyn Scene>
        })
    })
}

fn background(background: Option<Color>) -> impl Scene {
    scene_or_empty(background, |background| {
        background.map(|color| {
            Box::new(bsn! {
                BackgroundColor(color)
            }) as Box<dyn Scene>
        })
    })
}

/// Helper scene function to generate a button widget
pub fn button(text: &str, config: ButtonConfig) -> impl Scene {
    bsn! {
        @button_body(config.clone())
        Children [
            @button_text(text, config)
        ]
    }
}

/// Helper scene function to generate a button widget body
pub fn button_body(config: ButtonConfig) -> impl Scene {
    bsn! {
        Node {
            align_items: AlignItems::Center,
            border_radius: {config.border_radius},
            justify_content: JustifyContent::Center,
            padding: {config.padding},
            margin: {config.margin},
            min_width: {config.min_width},
        }
        @background(config.background_color)
        @shadow(config.box_shadows.clone())
        Button
    }
}

/// Helper scene function to generate a button widget inner text
pub fn button_text(text: &str, config: ButtonConfig) -> impl Scene {
    let font = config.font.clone();

    bsn! {
        Text(text)
        TextColor({config.text_color})
        // https://github.com/bevyengine/bevy/issues/25677#issuecomment-5690874259
        template(move |_| Ok(font.clone()))
        TextLayout::justify(Justify::Center)
    }
}

/// Helper scene function to generate a slider widget
pub fn slider(start: f32, end: f32, initial: f32, config: SliderConfig) -> impl Scene {
    bsn! {
        Node {
            align_items: AlignItems::Center,
            border_radius: {config.border_radius},
            height: {config.height},
            margin: {config.margin},
            width: Val::Percent(100.0),
        }
        @background(config.background_color)
        @shadow(config.box_shadows.clone())
        Slider
        SliderValue(initial)
        SliderRange::new(start, end)
        Children [
            @slider_thumb(config)
        ]
    }
}

/// Helper scene function to generate a slider widget thumb
pub fn slider_thumb(config: SliderConfig) -> impl Scene {
    bsn! {
        Node {
            border_radius: {config.thumb_border_radius},
            height: {config.thumb_size.y},
            position_type: PositionType::Absolute,
            width: {config.thumb_size.x},
        }
        @background(config.thumb_color)
        @shadow(config.box_shadows.clone())
        SliderThumb
    }
}

/// Helper scene function to generate a progress widget
pub fn progress(percent: f32, config: ProgressConfig) -> impl Scene {
    bsn! {
        Node {
            border_radius: {config.border_radius},
            height: {config.height},
            margin: {config.margin},
            overflow: Overflow::hidden(),
            width: Val::Percent(100.0),
        }
        @background(config.background_color)
        @shadow(config.box_shadows.clone())
        Progress(percent)
        Children [
            Node {
                position_type: PositionType::Absolute,
            }
            @background(config.fill_color)
            ProgressFill
        ]
    }
}
