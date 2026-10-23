use bevy::{
    color::{Color, Srgba},
    text::TextFont,
    ui::{BorderRadius, ShadowStyle, UiRect, Val, Val2},
    utils::default,
};

/// A struct to configure a button widget
#[derive(Clone, Debug, PartialEq)]
pub struct ButtonConfig {
    /// The background color of the button
    pub background_color: Color,
    /// The border radius of the button
    pub border_radius: BorderRadius,
    /// A vec of [ShadowStyle]s for the button
    pub box_shadows: Option<Vec<ShadowStyle>>,
    /// The font of the button
    pub font: TextFont,
    /// The margin of the button
    pub margin: UiRect,
    /// The minimum width of the button
    pub min_width: Val,
    /// The padding of the button
    pub padding: UiRect,
    /// The text color of the button
    pub text_color: Color,
}

impl Default for ButtonConfig {
    fn default() -> Self {
        Self {
            background_color: Color::BLACK,
            border_radius: default(),
            box_shadows: None,
            font: default(),
            margin: UiRect::AUTO,
            min_width: Val::Auto,
            padding: UiRect::AUTO,
            text_color: Color::WHITE,
        }
    }
}

/// A struct to configure a slider widget
#[derive(Clone, Debug, PartialEq)]
pub struct SliderConfig {
    /// The background color of the slider
    pub background_color: Color,
    /// The border radius of the slider
    pub border_radius: BorderRadius,
    /// A vec of [ShadowStyle]s for the slider
    pub box_shadows: Option<Vec<ShadowStyle>>,
    /// Enable this to implement your own custom slider thumb
    pub custom_thumb: bool,
    /// The height of the slider
    pub height: Val,
    /// The margin of the slider
    pub margin: UiRect,
    /// The border radius of the slider thumb
    pub thumb_border_radius: BorderRadius,
    /// The color of the slider thumb
    pub thumb_color: Color,
    /// The size of the slider thumb
    pub thumb_size: Val2,
}

impl Default for SliderConfig {
    fn default() -> Self {
        Self {
            background_color: Color::WHITE,
            border_radius: BorderRadius::MAX,
            box_shadows: None,
            custom_thumb: false,
            height: Val::Px(12.0),
            margin: UiRect::AUTO,
            thumb_color: Color::BLACK,
            thumb_size: Val2::all(Val::Px(24.0)),
            thumb_border_radius: BorderRadius::MAX,
        }
    }
}

/// A struct to configure a slider widget
#[derive(Clone, Debug, PartialEq)]
pub struct ProgressConfig {
    /// The background color of the progress bar
    pub background_color: Color,
    /// The border radius of the progress bar
    pub border_radius: BorderRadius,
    /// A vec of [ShadowStyle]s for the progress bar
    pub box_shadows: Option<Vec<ShadowStyle>>,
    /// The fill color of the progress bar
    pub fill_color: Color,
    /// The height of the progress bar
    pub height: Val,
    /// The margin of the progress bar
    pub margin: UiRect,
}

impl Default for ProgressConfig {
    fn default() -> Self {
        Self {
            background_color: Color::WHITE,
            border_radius: BorderRadius::all(Val::Px(10.0)),
            box_shadows: None,
            fill_color: Srgba::BLUE.into(),
            height: Val::Px(12.0),
            margin: UiRect::all(Val::Em(0.5)),
        }
    }
}
