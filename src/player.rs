use {
    super::{
        game_state::GameState,
        physics::{self, Acceleration, PhysicsBundle, TerminalVelocity},
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
        PhysicsBundle {
            friction: Friction::coefficient(10.),
            velocity: Velocity::zero(),
            terminal_velocity: TerminalVelocity(Vec2::splat(100.)),
            acceleration: Acceleration(Vec2::splat(0.)),
        },
    ));
}

pub fn player_movement(
    mut player_qry: Query<
        (&ActionState<PlayerAction>, &mut Velocity, &mut Acceleration),
        With<Player>,
    >,
) {
    let (player_actions, mut player_vel, mut player_acc) = player_qry.single_mut();

    if player_actions.pressed(PlayerAction::MoveLeft) {
        player_vel.linvel.x -= 10.;
        player_acc.0.x -= 1.;
    } else if player_actions.pressed(PlayerAction::MoveRight) {
        player_vel.linvel.x += 10.;
        player_acc.0.x += 1.;
    } else if player_acc.0.x.is_sign_positive() {
        player_acc.0.x = f32::max(player_acc.0.x - 1., 0.);
    } else if player_acc.0.x.is_sign_negative() {
        player_acc.0.x = f32::min(player_acc.0.x + 1., 0.);
    }
    if player_vel.linvel.x == 0. {
        player_acc.0.x = 0.;
    }
}
