use {
    super::{
        game_state::GameState,
        player::{self, Player},
        tile::{TileSize, TilemapSize},
    },
    bevy::prelude::*,
};

pub struct MainCameraPlugin;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::MIDNIGHT_BLUE))
            .add_systems(Startup, spawn_main_camera)
            .add_systems(
                Update,
                clamp_camera_to_tilemap.run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                FixedUpdate,
                follow_player
                    .after(player::player_movement)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Component)]
pub struct MainCamera;

fn spawn_main_camera(mut cmds: Commands) {
    let mut cam = Camera2dBundle::default();
    cam.projection.scale /= 10.;
    cmds.spawn((MainCamera, cam));
}

fn follow_player(
    mut main_cam_qry: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    player_qry: Query<&Transform, (With<Player>, Without<MainCamera>)>,
) {
    let mut main_cam_xform = main_cam_qry.single_mut();
    let player_xform = player_qry.single();

    main_cam_xform.translation = player_xform
        .translation
        .truncate()
        .extend(main_cam_xform.translation.z);
}

fn clamp_camera_to_tilemap(
    mut main_cam_qry: Query<(&Camera, &OrthographicProjection, &mut Transform), With<MainCamera>>,
    tile_size: Res<TileSize>,
    tilemap_size: Res<TilemapSize>,
) {
    let (main_cam, main_cam_projection, mut main_cam_xform) = main_cam_qry.single_mut();
    let Some(main_cam_viewport) = main_cam.logical_viewport_size() else {
        return;
    };

    let (tilemap_width_px, tilemap_height_px) = (
        tilemap_size.width as f32 * tile_size.width,
        tilemap_size.height as f32 * tile_size.height,
    );

    let (tilemap_half_width, tilemap_half_height) = (
        tilemap_size.width as f32 / 2.,
        tilemap_size.height as f32 / 2.,
    );
    let (tilemap_left, tilemap_right, tilemap_top, tilemap_bottom) = (
        -tilemap_half_width * tile_size.width - tile_size.width / 2.,
        tilemap_half_width * tile_size.width + tile_size.width / 2.,
        tilemap_half_height * tile_size.height + tile_size.height / 2.,
        -tilemap_half_height * tile_size.height - tile_size.height / 2.,
    );

    let correction_factor = if main_cam_projection.scale < 1. {
        main_cam_projection.scale.recip()
    } else {
        main_cam_projection.scale
    };
    let clamp_factor = 2. * correction_factor;

    if (tilemap_width_px / 2.) > (main_cam_viewport.x / clamp_factor) {
        main_cam_xform.translation.x = main_cam_xform.translation.x.clamp(
            tilemap_left + main_cam_viewport.x / clamp_factor,
            tilemap_right - main_cam_viewport.x / clamp_factor,
        );
    }
    if (tilemap_height_px / 2.) > (main_cam_viewport.y / clamp_factor) {
        main_cam_xform.translation.y = main_cam_xform.translation.y.clamp(
            tilemap_bottom + main_cam_viewport.y / clamp_factor,
            tilemap_top - main_cam_viewport.y / clamp_factor,
        );
    }
}
