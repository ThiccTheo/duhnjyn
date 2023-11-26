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
    for y in (-32..0).chain(1..=32) {
        for x in (-32..0).chain(1..=32) {
            if (y == -32 || y == 32) || (x == -32 || x == 32) {
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
