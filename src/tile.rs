use {super::game_state::GameState, bevy::prelude::*, bevy_rapier2d::prelude::*};

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_tiles);
    }
}

#[derive(Component)]
pub struct Tile;

fn spawn_tiles(mut cmds: Commands) {
    for y in (-5..0).chain(1..=5) {
        for x in (-5..0).chain(1..=5) {
            if (y == -5 || y == 5) || (x == -5 || x == 5) {
                let pos = Vec2::new(x as f32 * 16., y as f32 * 16.);
                cmds.spawn((
                    Tile,
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::RED,
                            custom_size: Some(Vec2::splat(16.)),
                            ..default()
                        },
                        transform: Transform::from_translation(pos.extend(1.)),
                        ..default()
                    },
                    Collider::cuboid(8., 8.),
                ));
            }
        }
    }
}
