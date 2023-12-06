use {
    super::{
        game_state::GameState,
        physics::{self, Acceleration, Grounded, NetDirection, TerminalVelocity},
        sprite_flip::Flippable,
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
                Update,
                discrete_player_input.run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                FixedUpdate,
                player_movement
                    .after(physics::process_collisions)
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

#[derive(Component, Default)]
pub struct Player {
    can_jump: bool,
}

fn spawn_player(mut cmds: Commands, assets: Res<AssetServer>) {
    cmds.spawn((
        Player::default(),
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
        Grounded::default(),
        Flippable::default(),
    ));
}

fn discrete_player_input(
    mut player_qry: Query<(&mut Player, &ActionState<PlayerAction>, &Grounded)>,
) {
    let (mut player, player_actions, player_grounded) = player_qry.single_mut();

    if player_actions.just_pressed(PlayerAction::Jump) && player_grounded.0 {
        player.can_jump = true;
    }
}

pub fn player_movement(
    mut player_qry: Query<(
        &mut Player,
        &ActionState<PlayerAction>,
        &mut Velocity,
        &mut NetDirection,
        &mut Grounded,
        &mut Flippable,
    )>,
) {
    let (
        mut player,
        player_actions,
        mut player_vel,
        mut player_net_dir,
        mut player_grounded,
        mut player_flippable,
    ) = player_qry.single_mut();

    if player_actions.released(PlayerAction::MoveLeft)
        && player_actions.released(PlayerAction::MoveRight)
    {
        player_net_dir.0.x = 0.;
    }
    if player_actions.pressed(PlayerAction::MoveLeft) {
        player_net_dir.0.x = -1.;
        player_flippable.flip_x = true;
    }
    if player_actions.pressed(PlayerAction::MoveRight) {
        player_net_dir.0.x = 1.;
        player_flippable.flip_x = false;
    }
    if player.can_jump {
        player.can_jump = false;
        player_grounded.0 = false;
        player_vel.linvel.y = 200.;
    }
}
