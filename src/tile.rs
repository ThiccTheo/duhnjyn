use {
    super::game_state::GameState, bevy::prelude::*, bevy_ecs_tilemap::prelude::*,
    bevy_rapier2d::prelude::*,
};

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_tiles);
    }
}

#[derive(Component)]
pub struct Tile;

fn spawn_tiles(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut tex_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let tilemap_size = TilemapSize { x: 32, y: 32 };
    let tilemap_id = cmds.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(tilemap_size);

    for y in 0..tilemap_size.y {
        for x in 0..tilemap_size.x {
            if (y == 0 || y == tilemap_size.y - 1) || (x == 0 || x == tilemap_size.x - 1) {
                let tile_pos = TilePos { x, y };
                let tile_id = cmds
                    .spawn(TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_id),
                        ..default()
                    })
                    .id();
                tile_storage.set(&tile_pos, tile_id);
            }
        }
    }
    let tile_size = TilemapTileSize { x: 16., y: 16. };
    let grid_size = TilemapGridSize::from(tile_size);

    cmds.entity(tilemap_id).insert(TilemapBundle {
        grid_size,
        size: tilemap_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(asset_server.load("tile.png")),
        tile_size,
        transform: get_tilemap_center_transform(&tilemap_size, &grid_size, &default(), 0.),
        spacing: TilemapSpacing::zero(),
        ..default()
    });
}
