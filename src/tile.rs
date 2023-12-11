use {super::game_state::GameState, bevy::prelude::*, bevy_rapier2d::prelude::*};

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TilemapSize {
            width: 10,
            height: 10,
        })
        .insert_resource(TileSize {
            width: 32.,
            height: 32.,
        })
        .add_systems(OnEnter(GameState::Playing), spawn_tiles);
    }
}

#[derive(Component)]
pub struct Tile;

#[derive(Resource)]
pub struct TilemapSize {
    pub width: usize,
    pub height: usize,
}

#[derive(Resource)]
pub struct TileSize {
    pub width: f32,
    pub height: f32,
}

fn spawn_tiles(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut tex_atlases: ResMut<Assets<TextureAtlas>>,
    tile_size: Res<TileSize>,
    tilemap_size: Res<TilemapSize>,
) {
    for y in (-5..1).chain(1..=5) {
        for x in (-5..1).chain(1..=5) {
            if (y == -5 || y == 5) || (x == -5 || x == 5) {
                let pos = Vec2::new(x as f32 * 32., y as f32 * 32.);
                cmds.spawn((
                    Tile,
                    SpriteSheetBundle {
                        sprite: TextureAtlasSprite {
                            index: 0,
                            custom_size: Some(Vec2::splat(64.)),
                            ..default()
                        },
                        texture_atlas: tex_atlases.add(TextureAtlas::from_grid(
                            asset_server.load("tile.png"),
                            Vec2::splat(32.),
                            3,
                            3,
                            None,
                            None,
                        )),
                        transform: Transform::from_translation(pos.extend(1.)),
                        ..default()
                    },
                    Collider::cuboid(16., 16.),
                ));
            }
        }
    }
}
