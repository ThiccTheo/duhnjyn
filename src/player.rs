use {
    super::{
        game_state::GameState,
        physics::{self, Acceleration, NetDirection, TerminalVelocity},
    },
    bevy::prelude::*,
    bevy_rapier2d::prelude::*,
    leafwing_input_manager::prelude::*,
    maplit::hashmap,
    std::collections::HashMap,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerInput(hashmap! {
            PlayerAction::MoveLeft => false,
            PlayerAction::MoveRight => false,
            PlayerAction::Jump => false,
        }))
        .add_systems(OnEnter(GameState::Playing), spawn_player)
        .add_systems(
            Update,
            poll_player_input.run_if(in_state(GameState::Playing)),
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

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub struct PlayerInput(HashMap<PlayerAction, bool>);

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

fn poll_player_input(
    player_qry: Query<&ActionState<PlayerAction>, With<Player>>,
    mut player_in: ResMut<PlayerInput>,
) {
    let player_actions = player_qry.single();

    player_in
        .0
        .get_mut(&PlayerAction::MoveLeft)
        .map(|pressed| *pressed = player_actions.pressed(PlayerAction::MoveLeft));

    player_in
        .0
        .get_mut(&PlayerAction::MoveRight)
        .map(|pressed| *pressed = player_actions.pressed(PlayerAction::MoveRight));

    player_in
        .0
        .get_mut(&PlayerAction::Jump)
        .map(|pressed| *pressed = player_actions.just_pressed(PlayerAction::Jump));
}

pub fn player_movement(
    mut player_qry: Query<(&mut Velocity, &mut NetDirection), With<Player>>,
    player_in: Res<PlayerInput>,
) {
    let (mut player_vel, mut player_net_dir) = player_qry.single_mut();

    if player_in.0[&PlayerAction::MoveLeft] {
        player_net_dir.0.x = -1.;
    }
    if player_in.0[&PlayerAction::MoveRight] {
        player_net_dir.0.x = 1.;
    }
    if !player_in.0[&PlayerAction::MoveLeft] && !player_in.0[&PlayerAction::MoveRight] {
        player_net_dir.0.x = 0.;
    }
    if player_in.0[&PlayerAction::Jump] && player_vel.linvel.y == 0. {
        player_vel.linvel.y = 200.;
    }
}
