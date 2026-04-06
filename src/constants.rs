use bevy::math::{UVec2, Vec2};

pub const PLAYER_SIZE: UVec2 = UVec2::new(68, 48);
pub const PLAYER_START_POSITION: Vec2 = Vec2::new(-500., 0.);
pub const PLAYER_JUMP_VELOCITY: f32 = 700.;
pub const PIPE_BASE_SPEED: f32 = 400.;
pub const GRAVITY: f32 = -2500.;
pub const BASE_PIPE_SPAWN_RATE: f32 = 1.1;
pub const BASE_PIPE_SPACE: f32 = 225.;
pub const PIPE_WIDTH: f32 = 132.;
pub const PIPE_HEIGHT: f32 = 796.;
pub const GROUND_HEIGHT: f32 = 100.;
pub const GROUND_SPRITE_HEIGHT: f32 = 176.;
pub const WINDOW_SIZE: Vec2 = Vec2::new(1920., 1080.);
pub const MINIMUM_PIPE_HEIGHT: f32 = 100.;
pub const BACKGROUND_SPRITE_HEIGHT: f32 = 1080.;

pub const BACKGROUND_Z: f32 = 0.;
pub const PIPE_Z: f32 = 1.;
pub const GROUND_Z: f32 = 2.;
pub const PLAYER_Z: f32 = 3.;
