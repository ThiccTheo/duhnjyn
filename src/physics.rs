use {super::game_state::GameState, bevy::prelude::*, bevy_rapier2d::prelude::*};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (zero_velocity_on_collision, apply_forces)
                .chain()
                .distributive_run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Component)]
pub struct TerminalVelocity(pub Vec2);

#[derive(Component)]
pub struct Acceleration(pub Vec2);

#[derive(Component)]
pub struct NetDirection(pub Vec2);

fn is_colliding_horizontally(normal: Vec2, threshold: f32) -> bool {
    let dot_prod = normal.normalize().dot(Vec2::X);
    dot_prod > threshold || dot_prod < -threshold
}

fn is_colliding_vertically(normal: Vec2, threshold: f32) -> bool {
    let dot_prod = normal.normalize().dot(Vec2::Y);
    dot_prod > threshold || dot_prod < -threshold
}

pub fn apply_forces(
    mut physics_qry: Query<(
        &mut KinematicCharacterController,
        &mut Velocity,
        &TerminalVelocity,
        &Friction,
        &Acceleration,
        &NetDirection,
    )>,
    time: Res<Time<Fixed>>,
) {
    let dt = time.timestep().as_secs_f32();

    for (mut kcc, mut vel, terminal_vel, friction, acc, net_dir) in physics_qry.iter_mut() {
        vel.linvel += acc.0 * net_dir.0 * dt;

        let dir = vel.linvel.normalize_or_zero();
        if dir.x > 0. {
            vel.linvel.x = f32::max(0., vel.linvel.x - friction.coefficient);
        } else if dir.x < 0. {
            vel.linvel.x = f32::min(vel.linvel.x + friction.coefficient, 0.);
        }
        vel.linvel.x = vel.linvel.x.clamp(-terminal_vel.0.x, terminal_vel.0.x);
        vel.linvel.y = vel.linvel.y.clamp(-terminal_vel.0.y, terminal_vel.0.y);

        let mut pos = Vec2::ZERO;
        pos += vel.linvel * dt;
        kcc.translation = Some(pos);
    }
}

pub fn zero_velocity_on_collision(
    mut physics_qry: Query<(&KinematicCharacterControllerOutput, &mut Velocity), With<Collider>>,
) {
    for (kcc_out, mut vel) in physics_qry.iter_mut() {
        for collision in kcc_out.collisions.iter() {
            let threshold = 0.8;

            if collision
                .toi
                .details
                .is_some_and(|details| is_colliding_horizontally(details.normal2, threshold))
            {
                vel.linvel.x = 0.;
            }
            if collision
                .toi
                .details
                .is_some_and(|details| is_colliding_vertically(details.normal2, threshold))
            {
                vel.linvel.y = 0.;
            }
        }
    }
}
