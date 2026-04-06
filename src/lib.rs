use bevy::prelude::*;

use crate::animation::animation_plugin;

pub mod animation;
pub mod components;
pub mod constants;
pub mod events;
pub mod resources;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    GameStart,
    InGame,
    GameOver,
}

pub fn flappy_bird_plugin(app: &mut App) {
    app.add_plugins(animation_plugin);
}
