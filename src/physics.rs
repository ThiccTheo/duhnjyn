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

#[derive(Bundle)]
pub struct PhysicsBundle {
    pub friction: Friction,
    pub velocity: Velocity,
    pub terminal_velocity: TerminalVelocity,
    pub acceleration: Acceleration,
}

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
        &Friction,
        &mut Velocity,
        &TerminalVelocity,
        &Acceleration,
    )>,
    time: Res<Time<Fixed>>,
) {
    let dt = time.timestep().as_secs_f32();

    for (mut kcc, friction, mut vel, terminal_vel, acc) in physics_qry.iter_mut() {
        let mut pos = Vec2::ZERO;
        let dir = Vec2::new(
            if vel.linvel.x == 0. {
                0.
            } else {
                vel.linvel.x.signum()
            },
            if vel.linvel.y == 0. {
                0.
            } else {
                vel.linvel.y.signum()
            },
        );
        let friction = friction.coefficient * -dir;
        let acc = acc.0;

        vel.linvel += acc * dt;
        vel.linvel += friction;
        if vel.linvel.x.signum() == friction.x.signum() {
            vel.linvel.x = 0.;
        }
        vel.linvel.x = vel.linvel.x.clamp(-terminal_vel.0.x, terminal_vel.0.x);
        vel.linvel.y = vel.linvel.y.clamp(-terminal_vel.0.y, terminal_vel.0.y);

        pos += vel.linvel * dt;
        println!("v(t) := <{}, {}>", vel.linvel.x, vel.linvel.y);
        println!("a(t) := <{}, {}>", acc.x, acc.y);
        println!("f := <{}, {}>", friction.x, friction.y);
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
