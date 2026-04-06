use bevy::{ecs::resource::Resource, time::Timer};

#[derive(Resource)]
pub struct PipeSpawnTimer(pub Timer);

#[derive(Resource, Debug)]
pub struct Score(pub i32);
