use bevy::ecs::event::Event;

#[derive(Event, Default)]
pub struct JumpEvent;

#[derive(Event)]
pub struct IncrementScoreEvent;

#[derive(Event)]
pub struct ResetScoreEvent;

#[derive(Event)]
pub struct UpdateScoreEvent {
    pub new_score: i32,
}

#[derive(Event)]
pub struct ScoreChangedEvent;

#[derive(Event)]
pub struct PipeCollisionEvent;

#[derive(Event)]
pub struct GroundCollisionEvent;
