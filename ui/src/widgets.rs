use bevy::{
    color::{Alpha, Color},
    ecs::{hierarchy::Children, template::template},
    scene::{Scene, bsn},
    text::{Justify, TextColor, TextFont, TextLayout},
    ui::{
        AlignItems, BackgroundColor, BorderRadius, BoxShadow, JustifyContent, Node, ShadowStyle,
        UiRect, Val, widget::Text,
    },
    utils::default,
};

use crate::Button;

/// A struct to style a button widget
#[derive(Clone, Debug, PartialEq)]
pub struct ButtonStyle {
    /// The minimum width of the button
    pub min_width: Val,
    /// The text color of the button
    pub text_color: Color,
    /// The background color of the button
    pub background_color: Color,
    /// The font of the button
    pub font: TextFont,
    /// The border radius of the button
    pub border_radius: BorderRadius,
    /// The padding of the button
    pub padding: UiRect,
    /// The margin of the button
    pub margin: UiRect,
    /// A vec of [ShadowStyle]s for the button
    pub box_shadows: Vec<ShadowStyle>,
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self {
            min_width: Val::Auto,
            text_color: Color::WHITE,
            background_color: Color::BLACK,
            font: default(),
            border_radius: BorderRadius::all(Val::Px(10.0)),
            padding: UiRect::all(Val::Em(0.5)),
            margin: UiRect::all(Val::Em(0.5)),
            box_shadows: vec![ShadowStyle {
                color: Color::BLACK.with_alpha(0.2),
                x_offset: Val::ZERO,
                y_offset: Val::ZERO,
                spread_radius: Val::Px(5.0),
                blur_radius: Val::Px(5.0),
            }],
        }
    }
}

/// Helper scene function to generate a button widget
pub fn button(text: &str, style: &ButtonStyle) -> impl Scene {
    bsn! {
        @button_body(style)
        Children [
            @button_text(text, style)
        ]
    }
}

/// Helper scene function to generate the body of a button widget
pub fn button_body(style: &ButtonStyle) -> impl Scene {
    bsn! {
        Node {
            align_items: AlignItems::Center,
            border_radius: {style.border_radius},
            justify_content: JustifyContent::Center,
            padding: {style.padding},
            margin: {style.margin},
            min_width: {style.min_width},
        }
        BackgroundColor({style.background_color})
        BoxShadow({style.box_shadows.clone()})
        Button
    }
}

/// Helper scene function to generate the inner text of a button widget
pub fn button_text(text: &str, style: &ButtonStyle) -> impl Scene {
    let font = style.font.clone();

    bsn! {
        Text(text)
        TextColor({style.text_color})
        // https://github.com/bevyengine/bevy/issues/25677#issuecomment-5690874259
        template(move |_| Ok(font.clone()))
        TextLayout {
            justify: Justify::Center,
        }
    }
}
