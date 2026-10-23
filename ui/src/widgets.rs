use bevy::{
    ecs::{hierarchy::Children, template::template},
    scene::{Scene, bsn},
    text::{Justify, TextColor, TextLayout},
    ui::{
        AlignItems, BackgroundColor, BoxShadow, JustifyContent, Node, PositionType, ShadowStyle,
        Val, widget::Text,
    },
    ui_widgets::{SliderRange, SliderThumb, SliderValue},
};
use functions::scene_or_empty;

use crate::{Button, ButtonConfig, Progress, ProgressConfig, ProgressFill, Slider, SliderConfig};

// fn shadow(shadow: &Option<Vec<ShadowStyle>>) -> Box<dyn Scene> {
//     match shadow {
//         Some(styles) => Box::new(bsn! {
//             BoxShadow({styles.clone()})
//         }),
//         None => Box::new(bsn! {}),
//     }
// }

fn shadow(shadow: Option<Vec<ShadowStyle>>) -> impl Scene {
    scene_or_empty(shadow, |shadow| {
        shadow.map(|styles| {
            Box::new(bsn! {
                BoxShadow(styles)
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
        BackgroundColor({config.background_color})
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
        TextLayout {
            justify: Justify::Center,
        }
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
        BackgroundColor({config.background_color})
        @shadow(config.box_shadows.clone())
        Slider
        SliderValue(initial)
        SliderRange::new(start, end)
        @scene_or_empty(!config.custom_thumb, |enabled| {
            if enabled {
                Some(Box::new(bsn! {
                    Children [
                        @slider_thumb(config)
                    ]
                }) as Box<dyn Scene>)
            } else {
                None
            }
        })
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
        BackgroundColor({config.thumb_color})
        @shadow(config.box_shadows.clone())
        SliderThumb
    }
}

/// Helper scene function to generate a progress widget
pub fn progress(percent: f32, config: ProgressConfig) -> impl Scene {
    bsn! {
        Node {
            height: {config.height},
            margin: {config.margin},
            width: Val::Percent(100.0),
        }
        BackgroundColor({config.background_color})
        @shadow(config.box_shadows.clone())
        Progress(percent)
        Children [
            BackgroundColor({config.fill_color})
            ProgressFill
        ]
    }
}
