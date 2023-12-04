use {
    super::{
        game_state::GameState,
        player::{self, Player},
    },
    bevy::prelude::*,
};

pub struct MainCameraPlugin;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_main_camera)
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
    cam.projection.scale /= 3.;
    cmds.spawn((MainCamera, cam));
}

fn follow_player(
    mut main_cam_qry: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    player_qry: Query<&Transform, (With<Player>, Without<MainCamera>)>,
) {
    let mut main_cam_xform = main_cam_qry.single_mut();
    let player_xform = player_qry.single();

    main_cam_xform.translation = Vec2::lerp(
        main_cam_xform.translation.truncate(),
        player_xform.translation.truncate(),
        0.1,
    )
    .extend(main_cam_xform.translation.z);
}
