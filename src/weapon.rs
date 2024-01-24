use {super::game_state::GameState, bevy::prelude::*, std::f32::consts::PI, std::time::Duration};

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
        //weapon_xform.rotation.z = weapon_xform.rotation.z.clamp(PI / 8., 2. * PI) * rotation_dir;
        weapon_xform.rotate_around(Vec3::new(-6., 6., 0.), Quat::from_rotation_z(5. * dt));
    }
}
