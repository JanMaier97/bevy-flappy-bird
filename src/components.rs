use bevy::{
    ecs::component::Component,
    math::Vec2,
    prelude::{Deref, DerefMut},
    time::Timer,
};

#[derive(PartialEq)]
pub enum ColliderType {
    Good,
    Bad,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Pipe;

#[derive(Component)]
pub struct PointGate;

#[derive(Component)]
pub struct Velocity(pub f32);

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct Collider {
    pub kind: ColliderType,
    pub size: Vec2,
}

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
