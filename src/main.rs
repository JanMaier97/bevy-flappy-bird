use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision}, log::LogPlugin, ecs::schedule::{LogLevel, ScheduleBuildSettings}, window::{PresentMode, WindowMode},
};
use rand::Rng;

const PLAYER_SIZE: Vec2 = Vec2::new(50., 50.);
const PLAYER_START_POSITION: Vec2 = Vec2::new(-500., 0.);
const PLAYER_DROP_SPEED: f32 = 300.;
const PLAYER_JUMP_VELOCITY: f32 = 700.;
const PLAYER_MASS: f32 = 1.0;
const PIPE_BASE_SPEED: f32 = 400.;
const GRAVITY: f32 = -2500.;
const BASE_PIPE_SPAWN_RATE: f32 = 1.;
const BASE_PIPE_SPACE: f32 = 150.;
const PIPE_WIDTH: f32 = 100.;
const GROUND_HEIGHT: f32 = 50.;
const WINDOW_SIZE: Vec2 = Vec2::new(1920., 1080.);
const MINIMUM_PIPE_HEIGHT: f32 = 100.;


#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, States)]
enum AppState{
    #[default]// todo: replace later with menu
    InGame,
    GameOver,
    Paused,
    MainMenu,
}

fn main() {
    App::new()
        .edit_schedule(CoreSchedule::Main, |schedule| {
            schedule.set_build_settings(ScheduleBuildSettings {
                ambiguity_detection: LogLevel::Warn,
                ..Default::default()
            });
        })
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
        .add_system(setup.in_schedule(OnEnter(AppState::InGame)))
        .add_systems((player_input, apply_gravity, pipe_spawner, pipe_movement).in_set(OnUpdate(AppState::InGame)))
        .add_system(game_over_input.in_set(OnUpdate(AppState::GameOver)))
        .add_system(detect_collision
            .run_if(in_state(AppState::InGame))
            .in_base_set(CoreSet::PostUpdateFlush))
        .add_system(bevy::window::close_on_esc)
        .run();
}

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

#[derive(Resource, Debug)]
struct Score(i32);

fn setup(mut commands: Commands) {
    commands.insert_resource(Score(0));
    commands.insert_resource(PipeSpawnTimer(Timer::from_seconds(
        BASE_PIPE_SPAWN_RATE,
        TimerMode::Repeating,
    )));
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(Player)
        .insert(Velocity(0.0))
        .insert(SpriteBundle {
            transform: Transform {
                scale: PLAYER_SIZE.extend(1.0),
                translation: PLAYER_START_POSITION.extend(1.0),
                ..default()
            },
            ..default()
        });

    let ground_y_pos = -WINDOW_SIZE.y / 2. + GROUND_HEIGHT / 2.;
    commands
        .spawn(Ground)
        .insert(Collider(ColliderType::Bad))
        .insert(SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0., ground_y_pos, 1.),
                scale: Vec3::new(WINDOW_SIZE.x, GROUND_HEIGHT, 1.),
                ..Default::default()
            },
            ..Default::default()
        });
}

fn apply_gravity(time: Res<Time>, mut query: Query<(&mut Transform, &mut Velocity), With<Player>>) {
    for (mut transform, mut velocity) in &mut query {
        // s = v_0 * t + 1/2 * a * t^2
        transform.translation.y +=
            velocity.0 * time.delta_seconds() + 0.5 * GRAVITY * time.delta_seconds().powi(2);

        // v = v_0 + a * t
        velocity.0 += GRAVITY * time.delta_seconds();
    }
}

fn player_input(
    mouse_input: Res<Input<MouseButton>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        info!("left mouse pressed");

        for mut velocity in &mut query {
            velocity.0 = PLAYER_JUMP_VELOCITY;
        }
    }
}

fn game_over_input(mut next_state: ResMut<NextState<AppState>>, key_input: Res<Input<KeyCode>>) {
    if key_input.just_pressed(KeyCode::R) {
        next_state.set(AppState::InGame);
    }
}

fn pipe_spawner(mut commands: Commands, mut spawn_timer: ResMut<PipeSpawnTimer>, time: Res<Time>) {
    if !spawn_timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let max_y_position = WINDOW_SIZE.y / 2. - BASE_PIPE_SPACE / 2. - MINIMUM_PIPE_HEIGHT ;
    let y_position = rand::thread_rng().gen_range(-max_y_position+GROUND_HEIGHT..=max_y_position);

    let top_pipe_height = WINDOW_SIZE.y / 2. - BASE_PIPE_SPACE / 2. - y_position;
    let top_pipe_position = top_pipe_height / 2. + BASE_PIPE_SPACE / 2.;

    let bottom_pipe_height = WINDOW_SIZE.y / 2. + BASE_PIPE_SPACE / 2. + y_position;
    let bottom_pipe_position = -bottom_pipe_height / 2. - BASE_PIPE_SPACE / 2.;

    let pipe_x_pos = WINDOW_SIZE.x / 2. + PIPE_WIDTH;

    commands
        .spawn(Pipe)
            .insert(SpatialBundle {
            transform: Transform {
                translation: Vec3::new(pipe_x_pos, y_position, 0.),
                ..default()
            },
            visibility: Visibility::Visible,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(Collider(ColliderType::Bad))
                .insert(SpriteBundle {
                    sprite: Sprite {
                        color: Color::GREEN,
                        ..default()
                    },
                    transform: Transform {
                        scale: Vec3::new(PIPE_WIDTH, top_pipe_height, 1.),
                        translation: Vec3::new(0., top_pipe_position, 0.),
                        ..default()
                    },
                    ..default()
                });

            parent
                .spawn(Collider(ColliderType::Bad))
                .insert(SpriteBundle {
                    sprite: Sprite {
                        color: Color::YELLOW,
                        ..default()
                    },
                    transform: Transform {
                        scale: Vec3::new(PIPE_WIDTH, bottom_pipe_height, 1.),
                        translation: Vec3::new(0., bottom_pipe_position, 0.),
                        ..default()
                    },
                    ..default()
                });

            parent
                .spawn(PointGate)
                .insert(Collider(ColliderType::Good))
                .insert(SpriteBundle {
                    transform: Transform {
                        scale: Vec3::new(10., BASE_PIPE_SPACE, 1.),
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
    mut score: ResMut<Score>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    player_query: Query<(&GlobalTransform, &Transform), With<Player>>,
    collider_query: Query<(Entity, &GlobalTransform, &Transform, &Collider)>,
) {
    for (player_global_transform, player_transform) in &player_query {
        for (collider_entity, collider_global_transform, collider_transform, collider) in &collider_query {
            let collision = collide(
                player_global_transform.translation(),
                player_transform.scale.truncate(),
                collider_global_transform.translation(),
                collider_transform.scale.truncate(),
            );

            let Some(collision) = collision else {
                continue;
            };

            info!("Collision found" );

            match collider.0 {
                ColliderType::Good => {
                    if collision != Collision::Right {
                        continue;
                    }
                    score.0 += 1;
                    commands.entity(collider_entity).despawn();
                    info!("Score: {}", score.0);
                }
                ColliderType::Bad => {
                    commands.entity(collider_entity).log_components();
                    info!("Collided with pipe or ground");
                    info!("Player global pos: {}", player_global_transform.translation());
                    info!("Ground global pos: {}", collider_global_transform.translation());
                    info!("Player size: {}", player_transform.scale);
                    info!("Ground size: {}", collider_transform.scale);
                    next_state.set(AppState::GameOver);
                }
            }
        }
    }
}
