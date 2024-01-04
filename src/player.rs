use {
    super::{
        animation::{self, AnimationIndices, AnimationTimer},
        game_state::GameState,
        mouse_position::MousePosition,
        physics::{self, Acceleration, Grounded, NetDirection, TerminalVelocity},
        sprite_flip::Flippable,
        weapon::Weapon,
    },
    bevy::{prelude::*, sprite::Anchor},
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
    Attack,
}

#[derive(Component, Default)]
pub struct Player {
    can_jump: bool,
}

#[derive(Component)]
pub struct PlayerCamera;

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
                Vec2::splat(32.),
                5,
                4,
                None,
                None,
            )),
            transform: Transform::from_xyz(0., 0., 2.),
            ..default()
        },
        InputManagerBundle::<PlayerAction> {
            input_map: InputMap::new([
                (KeyCode::A, PlayerAction::MoveLeft),
                (KeyCode::D, PlayerAction::MoveRight),
                (KeyCode::Space, PlayerAction::Jump),
            ])
            .insert(MouseButton::Left, PlayerAction::Attack)
            .clone(),
            ..default()
        },
        KinematicCharacterController::default(),
        Collider::capsule_y(4.5, 6.),
        Friction::coefficient(3.),
        Velocity::zero(),
        TerminalVelocity(Vec2::new(50., 200.)),
        Acceleration(Vec2::new(300., 500.)),
        NetDirection { x: 0, y: -1 },
        Grounded::default(),
        Flippable::default(),
        AnimationIndices { first: 0, last: 0 },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ))
    .with_children(|parent| {
        let mut player_cam = Camera2dBundle::default();
        player_cam.projection.scale /= 3.;
        parent.spawn((PlayerCamera, player_cam));

        parent.spawn((
            SpriteBundle {
                sprite: Sprite {
                    anchor: Anchor::BottomLeft,
                    ..default()
                },
                transform: Transform::from_xyz(0., 0., 10.),
                texture: asset_server.load("sword.png"),
                ..default()
            },
            Flippable::default(),
            Weapon,
            Name::new("Sword"),
        ));
    });
}

fn update_animation_state(
    mut player_qry: Query<
        (
            &mut AnimationIndices,
            &TextureAtlasSprite,
            &Grounded,
            &NetDirection,
            &ActionState<PlayerAction>,
        ),
        With<Player>,
    >,
) {
    let (
        mut player_animation_indices,
        player_tex_atlas_sprite,
        player_grounded,
        player_net_dir,
        player_actions,
    ) = player_qry.single_mut();

    let jumping = AnimationIndices { first: 5, last: 5 };
    let walking = AnimationIndices { first: 6, last: 19 };
    let idling = AnimationIndices { first: 0, last: 0 };
    let attacking = AnimationIndices { first: 1, last: 4 };

    let attack_in_progress = *player_animation_indices == attacking
        && player_animation_indices.last != player_tex_atlas_sprite.index;

    if player_actions.pressed(PlayerAction::Attack) {
        if *player_animation_indices != attacking {
            *player_animation_indices = attacking;
        }
    } else if !player_grounded.0 {
        if *player_animation_indices != jumping && !attack_in_progress {
            *player_animation_indices = jumping;
        }
    } else if player_net_dir.x != 0 {
        if *player_animation_indices != walking && !attack_in_progress {
            *player_animation_indices = walking;
        }
    } else if *player_animation_indices != idling && !attack_in_progress {
        *player_animation_indices = idling;
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
        &Transform,
        &mut Velocity,
        &mut NetDirection,
        &mut Grounded,
        &mut Flippable,
    )>,
    mouse_pos: Res<MousePosition>,
) {
    let (
        mut player,
        player_actions,
        player_xform,
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
    if player_actions.pressed(PlayerAction::Attack) {
        player_flippable.flip_x = player_xform.translation.truncate().x > mouse_pos.x;
    }
    if player.can_jump {
        player.can_jump = false;
        player_grounded.0 = false;
        player_vel.linvel.y = 200.;
    }
}
