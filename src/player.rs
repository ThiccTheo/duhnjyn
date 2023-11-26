use {
    super::{game_state::GameState, physics::Acceleration}, bevy::prelude::*, bevy_rapier2d::prelude::*,
    leafwing_input_manager::prelude::*,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(
                FixedUpdate,
                player_movement.run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Actionlike, TypePath, Clone)]
pub enum PlayerAction {
    MoveLeft,
    MoveRight,
    Jump,
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut cmds: Commands, assets: Res<AssetServer>) {
    cmds.spawn((
        Player,
        SpriteBundle {
            texture: assets.load("player.png"),
            ..default()
        },
        InputManagerBundle::<PlayerAction> {
            input_map: InputMap::new([
                (KeyCode::A, PlayerAction::MoveLeft),
                (KeyCode::D, PlayerAction::MoveRight),
            ]),
            ..default()
        },
        KinematicCharacterController::default(),
        Collider::cuboid(6., 21. / 2.),
        Velocity::default(),
        Acceleration(Vec2::splat(100.)),
    ));
}

pub fn player_movement(
    mut player_qry: Query<
        (
            &ActionState<PlayerAction>,
            &mut KinematicCharacterController,
        ),
        With<Player>,
    >,
) {
    let (player_actions, mut player_kcc) = player_qry.single_mut();
    let mut move_amt = Vec2::ZERO;

    if player_actions.pressed(PlayerAction::MoveLeft) {
        move_amt.x = -10.;
    }
    if player_actions.pressed(PlayerAction::MoveRight) {
        move_amt.x = 10.;
    }
    player_kcc.translation = Some(move_amt);
}
