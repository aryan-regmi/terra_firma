use bevy::prelude::*;
use bevy_inspector_egui::egui;

pub(crate) mod animation;
pub(crate) mod maps;

pub(crate) use animation::*;
pub(crate) use maps::*;

/// A name `Component`.
#[derive(Component, Reflect, Default, PartialEq, Eq, Hash)]
pub(crate) struct Name(pub(crate) String);

impl Into<String> for Name {
    fn into(self) -> String {
        self.0
    }
}

impl From<&str> for Name {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

/// Triggers when the game has been resumed.
#[derive(Event)]
pub(crate) struct ResumeGameEvent;

/// Creates a button with the given label and size.
pub(crate) fn sized_button(
    ui: &mut egui::Ui,
    label: &str,
    width: f32,
    height: f32,
) -> egui::Response {
    ui.add_sized((width, height), egui::Button::new(label))
}
