use {
    super::{
        game_state::GameState,
        physics::{self, Acceleration, NetDirection, TerminalVelocity},
    },
    bevy::prelude::*,
    bevy_rapier2d::prelude::*,
    leafwing_input_manager::prelude::*,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(
                FixedUpdate,
                player_movement
                    .after(physics::zero_velocity_on_collision)
                    .before(physics::apply_forces)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Actionlike, Clone, Eq, PartialEq, Hash, Reflect)]
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
                (KeyCode::Space, PlayerAction::Jump),
            ]),
            ..default()
        },
        KinematicCharacterController::default(),
        Collider::cuboid(6., 21. / 2.),
        Friction::coefficient(2.),
        Velocity::zero(),
        TerminalVelocity(Vec2::new(100., 200.)),
        Acceleration(Vec2::new(300., 500.)),
        NetDirection(Vec2::new(0., -1.)),
    ));
}

pub fn player_movement(
    mut player_qry: Query<
        (&ActionState<PlayerAction>, &mut Velocity, &mut NetDirection),
        With<Player>,
    >,
) {
    let (player_actions, mut player_vel, mut player_net_dir) = player_qry.single_mut();

    if player_actions.pressed(PlayerAction::MoveLeft) {
        player_net_dir.0.x = -1.;
    }
    if player_actions.pressed(PlayerAction::MoveRight) {
        player_net_dir.0.x = 1.;
    }
    if !player_actions.pressed(PlayerAction::MoveLeft)
        && !player_actions.pressed(PlayerAction::MoveRight)
    {
        player_net_dir.0.x = 0.;
    }
    if player_actions.just_pressed(PlayerAction::Jump) && player_vel.linvel.y == 0. {
        player_vel.linvel.y = 200.;
    }
}
