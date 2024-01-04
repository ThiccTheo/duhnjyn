use {super::game_state::GameState, bevy::prelude::*, std::time::Duration};

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, swing_weapons.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct Weapon;

fn swing_weapons(time: Res<Time>, mut weapon_qry: Query<&mut Transform, With<Weapon>>) {
    let dt = time.delta_seconds();

    for mut weapon_xform in weapon_qry.iter_mut() {
        let rotation_dir =
            -(weapon_xform.translation.x.signum() * weapon_xform.translation.y.signum());
        weapon_xform.rotate_z(0.1 * rotation_dir * dt);
    }
}
