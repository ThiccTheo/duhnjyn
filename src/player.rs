use {
    super::{
        animation::{self, AnimationIndices, AnimationTimer},
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
                (
                    discrete_player_input,
                    update_animation_state.before(animation::adjust_sprite_indices),
                )
                    .run_if(in_state(GameState::Playing)),
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

fn spawn_player(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut tex_atlases: ResMut<Assets<TextureAtlas>>,
) {
    cmds.spawn((
        Player::default(),
        Name::new("Player"),
        SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 0,
                ..default()
            },
            texture_atlas: tex_atlases.add(TextureAtlas::from_grid(
                asset_server.load("player.png"),
                Vec2::splat(64.),
                5,
                4,
                None,
                None,
            )),
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
        Collider::cuboid(24. / 2., 42. / 2.),
        Friction::coefficient(2.),
        Velocity::zero(),
        TerminalVelocity(Vec2::new(100., 200.)),
        Acceleration(Vec2::new(300., 500.)),
        NetDirection { x: 0, y: -1 },
        Grounded::default(),
        Flippable::default(),
        AnimationIndices { first: 0, last: 0 },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
    // .with_children(|player| {
    //     player.spawn((
    //         SpriteBundle {
    //             transform: Transform::from_xyz(1., 5., 1.),
    //             texture: asset_server.load("sclera.png"),
    //             ..default()
    //         },
    //         Flippable::default(),
    //     ));
    //     player.spawn((
    //         SpriteBundle {
    //             transform: Transform::from_xyz(1., 5., 1.),
    //             texture: asset_server.load("iris.png"),
    //             ..default()
    //         },
    //         Flippable::default(),
    //     ));
    // });
}

fn update_animation_state(
    mut player_qry: Query<(&mut AnimationIndices, &Grounded, &NetDirection), With<Player>>,
) {
    let (mut player_animation_indices, player_grounded, player_net_dir) = player_qry.single_mut();

    // tmp
    let jumping = AnimationIndices { first: 5, last: 5 };
    let walking = AnimationIndices { first: 6, last: 19 };
    let idle = AnimationIndices { first: 0, last: 0 };

    if !player_grounded.0 && *player_animation_indices != jumping {
        *player_animation_indices = jumping;
    } else if player_grounded.0 && (player_net_dir.x == -1 || player_net_dir.x == 1) && *player_animation_indices != walking {
        *player_animation_indices = walking;
    } else if player_grounded.0 && player_net_dir.x == 0 && *player_animation_indices != idle {
        *player_animation_indices = idle;
    }
    
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
        player_net_dir.x = 0;
    }
    if player_actions.pressed(PlayerAction::MoveLeft) {
        player_net_dir.x = -1;
        player_flippable.flip_x = true;
    }
    if player_actions.pressed(PlayerAction::MoveRight) {
        player_net_dir.x = 1;
        player_flippable.flip_x = false;
    }
    if player.can_jump {
        player.can_jump = false;
        player_grounded.0 = false;
        player_vel.linvel.y = 200.;
    }
}
