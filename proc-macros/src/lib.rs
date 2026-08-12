use proc_macro::TokenStream;

/// Macro to create a marker `Component`
#[proc_macro_attribute]
pub fn marker_component_bundle(_: TokenStream, item: TokenStream) -> TokenStream {
    let item_code = item.to_string();
    let bundled = format!(
        "#[derive(Clone, bevy::ecs::component::Component, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]\n{}",
        item_code
    );

    bundled.parse().unwrap()
}

/// Macro to create a `State`
#[proc_macro_attribute]
pub fn state_bundle(_: TokenStream, item: TokenStream) -> TokenStream {
    let item_code = item.to_string();
    let bundled = format!(
        "#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, bevy::prelude::States)]\n{}",
        item_code
    );

    bundled.parse().unwrap()
}
