use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    window::WindowMode, animation,
};
use rand::Rng;
use std::f32::consts::PI;

// const PLAYER_SIZE: Vec2 = Vec2::new(136., 96.);
const PLAYER_SIZE: Vec2 = Vec2::new(68., 48.);
const PLAYER_START_POSITION: Vec2 = Vec2::new(-500., 0.);
const PLAYER_JUMP_VELOCITY: f32 = 700.;
const PIPE_BASE_SPEED: f32 = 400.;
const GRAVITY: f32 = -2500.;
const BASE_PIPE_SPAWN_RATE: f32 = 1.1;
const BASE_PIPE_SPACE: f32 = 225.;
const PIPE_WIDTH: f32 = 132.;
const PIPE_HEIGHT: f32 = 796.;
const GROUND_HEIGHT: f32 = 100.;
const GROUND_SPRITE_HEIGHT: f32 = 176.;
const WINDOW_SIZE: Vec2 = Vec2::new(1920., 1080.);
const MINIMUM_PIPE_HEIGHT: f32 = 100.;
const BACKGROUND_SPRITE_HEIGHT: f32 = 1080.;

const BACKGROUND_Z: f32 = 0.;
const PIPE_Z: f32 = 1.;
const GROUND_Z: f32 = 2.;
const PLAYER_Z: f32 = 3.;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    GameStart,
    InGame,
    GameOver,
    Paused,
    MainMenu,
}

fn main() {
    App::new()
        .add_event::<JumpEvent>()
        .add_event::<CollisionEvent>()
        .add_event::<IncrementScoreEvent>()
        .add_event::<ScoreChangedEvent>()
        .add_event::<UpdateScoreEvent>()
        .add_event::<PipeCollisionEvent>()
        .add_event::<GroundCollisionEvent>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Flappy Bird".into(),
                // resolution: (1920., 1080.).into(),
                mode: WindowMode::Fullscreen,
                // resizable: false,
                focused: true,
                position: WindowPosition::Centered(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .add_state::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(OnEnter(AppState::GameStart), spawn_player)
        .add_systems(
            Update,
            (trigger_game_start, idle_player_movement).run_if(in_state(AppState::GameStart)),
        )
        .add_systems(
            Update,
            (
                player_input,
                animate_sprite, 
                apply_gravity,
                pipe_spawner,
                pipe_movement,
                apply_jump_velocity,
            )
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(Update, (update_score, update_score_text))
        .add_systems(
            PostUpdate,
            detect_collision.run_if(in_state(AppState::InGame)),
        )
        .add_systems(Update, game_over_input.run_if(in_state(AppState::GameOver)))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

#[derive(Event, Default)]
struct JumpEvent;

#[derive(Event)]
struct CollisionEvent {
    entity: Entity,
    collision: Collision,
}

#[derive(Event)]
struct IncrementScoreEvent;

#[derive(Event)]
struct ResetScoreEvent;

#[derive(Event)]
struct UpdateScoreEvent {
    new_score: i32,
}

#[derive(Event)]
struct ScoreChangedEvent;

#[derive(Event)]
struct PipeCollisionEvent;

#[derive(Event)]
struct GroundCollisionEvent;

#[derive(PartialEq)]
enum ColliderType {
    Good,
    Bad,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Pipe;

#[derive(Component)]
struct PointGate;

#[derive(Component)]
struct Velocity(f32);

#[derive(Component)]
struct Ground;

#[derive(Component)]
struct Collider(ColliderType);

#[derive(Resource)]
struct PipeSpawnTimer(Timer);

#[derive(Component)]
struct ScoreText;

#[derive(Resource, Debug)]
struct Score(i32);

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn setup(mut commands: Commands, asset_server: Res<AssetServer> ) {
    commands.insert_resource(Score(0));
    commands.insert_resource(PipeSpawnTimer(Timer::from_seconds(
        BASE_PIPE_SPAWN_RATE,
        TimerMode::Repeating,
    )));

    commands.spawn(Camera2dBundle::default());


    let background_handle = asset_server.load("background.png");
    let background_y_pos = -WINDOW_SIZE.y / 2. + GROUND_HEIGHT + BACKGROUND_SPRITE_HEIGHT/2.;
    commands.spawn(SpriteBundle{
        texture: background_handle,
        transform: Transform {
            translation: Vec3::new(0., background_y_pos, BACKGROUND_Z),
            ..Default::default()
        },
        ..Default::default()
    });

    let ground_handle: Handle<Image> = asset_server.load("ground.png");
    let ground_sprite_y_pos =  -WINDOW_SIZE.y / 2. + GROUND_HEIGHT  - GROUND_SPRITE_HEIGHT / 2.;
    commands
        .spawn(Ground)
        .insert(Collider(ColliderType::Bad))
        .insert(SpriteBundle {
            texture: ground_handle,
            transform: Transform {
                translation: Vec3::new(0., ground_sprite_y_pos, GROUND_Z),
                ..Default::default()
            },
            ..Default::default()
        });

    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "0",
            TextStyle {
                font_size: 50.0,
                color: Color::ORANGE,
                ..default()
            },
        ) // Set the alignment of the Text
            .with_text_alignment(TextAlignment::Center)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(100.0),
                right: Val::Px(100.0),
                ..default()
            }),
        ScoreText,
    ));
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    // let texture_handle = asset_server.load("bird.png");
    let texture_handle = asset_server.load("bird_old.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, PLAYER_SIZE, 3, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices{ first: 0, last: 2 };

    commands
        .spawn(Player)
        .insert(Velocity(0.0))
        .insert(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(1),
            transform: Transform {
                translation: PLAYER_START_POSITION.extend(PLAYER_Z),
                scale: Vec2::splat(1.).extend(1.),
                ..default()
            },
            ..default()
        })
        .insert(animation_indices)
        .insert(AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)));
}

fn apply_gravity(time: Res<Time>, mut query: Query<(&mut Transform, &mut Velocity), With<Player>>) {
    for (mut transform, mut velocity) in &mut query {
        // s = v_0 * t + 1/2 * a * t^2
        transform.translation.y +=
            velocity.0 * time.delta_seconds() + 0.5 * GRAVITY * time.delta_seconds().powi(2);

        transform.translation.y = transform
            .translation
            .y
            .min(WINDOW_SIZE.y / 2. + PLAYER_SIZE.y / 2.);

        // v = v_0 + a * t
        velocity.0 += GRAVITY * time.delta_seconds();
    }
}

fn trigger_game_start(
    mouse_input: Res<Input<MouseButton>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut jump_event: EventWriter<JumpEvent>,
) {
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }

    jump_event.send_default();
    next_state.set(AppState::InGame);
}

fn idle_player_movement(
    mut player_transform_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let frequency = 0.5;
    let amplitude = 10.;
    let wave_position = 2. * PI * frequency * time.elapsed_seconds();
    let translation = amplitude * wave_position.sin();

    for mut transform in player_transform_query.iter_mut() {
        transform.translation.y = translation;
    }
}

fn player_input(mouse_input: Res<Input<MouseButton>>, mut jump_event: EventWriter<JumpEvent>) {
    if mouse_input.just_pressed(MouseButton::Left) {
        info!("left mouse pressed");

        jump_event.send_default();
    }
}

fn apply_jump_velocity(
    mut jump_events: EventReader<JumpEvent>,
    mut player_velocity_query: Query<&mut Velocity, With<Player>>,
) {
    for _ in jump_events.read() {
        for mut velocity in player_velocity_query.iter_mut() {
            velocity.0 = PLAYER_JUMP_VELOCITY;
        }
    }
}

fn game_over_input(
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    mut event_writer: EventWriter<UpdateScoreEvent>,
    player_query: Query<Entity, With<Player>>,
    pipes_query: Query<Entity, With<Pipe>>,
    key_input: Res<Input<KeyCode>>,
) {
    if !key_input.just_pressed(KeyCode::R) {
        return;
    }

    for player in player_query.iter() {
        commands.entity(player).despawn()
    }

    for pipe in pipes_query.iter() {
        commands.entity(pipe).despawn_recursive()
    }

    event_writer.send(UpdateScoreEvent { new_score: 0 });

    next_state.set(AppState::GameStart);
}

fn pipe_spawner(mut commands: Commands, mut spawn_timer: ResMut<PipeSpawnTimer>, time: Res<Time>, asset_server: Res<AssetServer>) {
    if !spawn_timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let max_opening_y_pos = WINDOW_SIZE.y / 2. - MINIMUM_PIPE_HEIGHT - BASE_PIPE_SPACE / 2.;
    let min_opening_y_pos = -max_opening_y_pos + GROUND_HEIGHT;
    let pipe_group_center = rand::thread_rng().gen_range(min_opening_y_pos..=max_opening_y_pos);

    let pipe_offset = BASE_PIPE_SPACE/2. + PIPE_HEIGHT /2.; 


    let pipe_x_pos = WINDOW_SIZE.x / 2. + PIPE_WIDTH;


    let texture_handle = asset_server.load("pipe.png");
    commands
        .spawn(Pipe)
        .insert(SpatialBundle {
            transform: Transform {
                translation: Vec3::new(pipe_x_pos, pipe_group_center, PIPE_Z),
                ..default()
            },
            visibility: Visibility::Visible,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(Collider(ColliderType::Bad))
                .insert(SpriteBundle {
                    texture: texture_handle.clone(),
                    transform: Transform {
                        translation: Vec3::new(0., pipe_offset, 0.),
                        rotation: Quat::from_rotation_x(PI),
                        ..default()
                    },
                    ..default()
                });

            parent
                .spawn(Collider(ColliderType::Bad))
                .insert(SpriteBundle {
                    texture: texture_handle.clone(),
                    transform: Transform {
                        translation: Vec3::new(0., -pipe_offset, 0.),
                        ..default()
                    },
                    ..default()
                });

            parent
                .spawn(PointGate)
                .insert(Collider(ColliderType::Good))
                .insert(SpriteBundle {
                    transform: Transform {
                        scale: Vec3::new(10., BASE_PIPE_SPACE, 0.),
                        ..Default::default()
                    },
                    sprite: Sprite {
                        color: Color::RED,
                        ..default()
                    },
                    ..default()
                });
        });
}

fn pipe_movement(time: Res<Time>, mut query: Query<&mut Transform, With<Pipe>>) {
    for mut pipe_transform in &mut query {
        pipe_transform.translation.x -= PIPE_BASE_SPEED * time.delta_seconds();
    }
}

fn detect_collision(
    score: Res<Score>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    mut collision_event: EventWriter<CollisionEvent>,
    mut score_event_writer: EventWriter<UpdateScoreEvent>,
    player_query: Query<(&GlobalTransform, &Transform), With<Player>>,
    collider_query: Query<(Entity, &GlobalTransform, &Transform, &Collider)>,
) {
    for (player_global_transform, player_transform) in &player_query {
        for (collider_entity, collider_global_transform, collider_transform, collider) in
            &collider_query
        {
            let collision = collide(
                player_global_transform.translation(),
                player_transform.scale.truncate(),
                collider_global_transform.translation(),
                collider_transform.scale.truncate(),
            );

            let Some(collision) = collision else {
                continue;
            };

            collision_event.send(CollisionEvent {
                entity: collider_entity,
                collision,
            });

            match collider.0 {
                ColliderType::Good => {
                    if collision != Collision::Right {
                        continue;
                    }

                    score_event_writer.send(UpdateScoreEvent { new_score: score.0 +1 });
                    commands.entity(collider_entity).despawn();
                }
                ColliderType::Bad => {
                    next_state.set(AppState::GameOver);
                }
            }
        }
    }
}


fn update_score(
    mut score: ResMut<Score>,
    mut update_event: EventReader<UpdateScoreEvent>,
    mut event_writer: EventWriter<ScoreChangedEvent>,
) {
    if !update_event.is_empty() {
        event_writer.send(ScoreChangedEvent);
    } else {
    }

    for event in update_event.read() {
        score.0 = event.new_score;
    }


}

fn update_score_text(
    score: Res<Score>,
    mut change_event: EventReader<ScoreChangedEvent>,
    mut query: Query<&mut Text, With<ScoreText>>,
) {
    for _ in change_event.read() {
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("{}", score.0);
        }
    }
}


fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}