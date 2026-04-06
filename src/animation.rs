use bevy::prelude::*;

use crate::{
    components::{AnimationIndices, AnimationTimer},
    AppState,
};

pub fn animation_plugin(app: &mut App) {
    app.add_systems(
        Update,
        animate_sprite.run_if(in_state(AppState::InGame).or(in_state(AppState::GameStart))),
    );
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if !timer.just_finished() {
            continue;
        }

        let Some(atlas) = &mut sprite.texture_atlas else {
            continue;
        };

        atlas.index = if atlas.index == indices.last {
            indices.first
        } else {
            atlas.index + 1
        };
    }
}
