use crate::{
    board::{BoardCache, BoardPosition, Board},
    pawn::Player,
    Game,
};
use bevy::prelude::*;
use rand::Rng;

pub struct PickupPlugin;

impl Plugin for PickupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                pickup_collide,
                expire_pickups,
                rotate_pickups,
                respawn_pickups,
            )
                .chain(),
        );
    }
}

/// Marker for components that can be pickup up
#[derive(Component)]
pub struct Pickup;

/// Entities with this component will be reset to another BoardPosition when the timer expires
#[derive(Component)]
pub struct AutoRespawn(pub Timer);

/// Marked for respawning by AutoRespawn
#[derive(Component)]
pub struct Despawn;

pub fn spawn_a_pickup(
    commands: &mut Commands,
    pickup_handle: Handle<Scene>,
    x: usize,
    y: usize,
    height: f32,
) {
    commands
        .spawn((
            Pickup,
            AutoRespawn(Timer::from_seconds(5.0, TimerMode::Once)),
            BoardPosition {
                x: x as usize,
                y: y as usize,
            },
            SceneBundle {
                transform: Transform::from_xyz(x as f32, height, y as f32),
                scene: pickup_handle,
                ..default()
            },
        ))
        .with_children(|children| {
            children.spawn(PointLightBundle {
                point_light: PointLight {
                    color: Color::rgb(1.0, 1.0, 0.0),
                    intensity: 1000.0,
                    range: 10.0,
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 2.0, 0.0),
                ..default()
            });
        });
}

/// respawn pickups that are marked for removal
fn respawn_pickups(
    mut commands: Commands,
    game: ResMut<Game>,
    boards: Res<Assets<Board>>,
    players: Query<&BoardPosition, With<Player>>,
    pickups: Query<(&BoardPosition, Entity), (With<Pickup>, With<Despawn>, With<AutoRespawn>)>,
    gameboard: Query<&BoardCache>,
) {
    let Some(board_config) = boards.get(game.world_handle.clone()) else {
        warn!("expected board config asset to be loaded by now");
        return;
    };
    let mut restricted_coordinates = players
        .iter()
        .map(|p| (p.x, p.y))
        .chain(pickups.iter().map(|(p, _)| (p.x, p.y)))
        .collect::<Vec<_>>();

    for (_, entity) in pickups.iter() {
        commands.entity(entity).despawn_recursive();

        let Ok(board_cache) = gameboard.get_single() else {
            warn!("unexpected number of game boards");
            return;
        };

        let pickup_coord = loop {
            let x = rand::thread_rng().gen_range(0..board_config.width);
            let y = rand::thread_rng().gen_range(0..board_config.height);
            if !restricted_coordinates.contains(&(x, y)) {
                restricted_coordinates.push((x, y));
                break BoardPosition { x, y };
            }
        };

        spawn_a_pickup(
            &mut commands,
            game.pickup_handle.clone(),
            pickup_coord.x,
            pickup_coord.y,
            board_cache.get_height(pickup_coord.x, pickup_coord.y) + 0.2,
        );
    }
}

/// Trigger pickups to automatically despawn if their timer expires
fn expire_pickups(
    time: Res<Time>,
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut pickups: Query<(&mut AutoRespawn, &BoardPosition, Entity), With<Pickup>>,
) {
    for (mut timer, _bonus, entity) in pickups.iter_mut() {
        if !timer.0.tick(time.delta()).finished() {
            continue;
        }
        game.score -= 10;
        commands.entity(entity).insert(Despawn);
    }
}

/// cause pickups to rotate
///
/// and scale pickups according to score
fn rotate_pickups(time: Res<Time>, mut pickups: Query<&mut Transform, With<Pickup>>) {
    for mut cake_transform in pickups.iter_mut() {
        cake_transform.rotate_y(time.delta_seconds());
        cake_transform.scale = Vec3::splat(1.0 + (1.0 * time.elapsed_seconds().sin()).abs());
    }
}

/// process input to move the player pawn
pub fn pickup_collide(
    mut commands: Commands,
    mut game: ResMut<Game>,
    players: Query<&BoardPosition, With<Player>>,
    bonus: Query<(&BoardPosition, Entity), (With<Pickup>, (Without<Player>, Without<Despawn>))>,
) {
    for player_position in players.iter() {
        for (bonus, entity) in bonus.iter() {
            if *player_position == *bonus {
                game.score += 1;
                game.cake_eaten += 1;
                commands.entity(entity).insert(Despawn);
            }
        }
    }
}
