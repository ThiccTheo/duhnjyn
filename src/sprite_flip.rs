use {super::game_state::GameState, bevy::prelude::*};

pub struct SpriteFlipPlugin;

impl Plugin for SpriteFlipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (propagate_sprite_flips, convert_flippables_to_sprite_flips)
                .chain()
                .distributive_run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Component, Default)]
pub struct Flippable {
    pub flip_x: bool,
    pub flip_y: bool,
}

impl Flippable {
    pub fn new(flip_x: bool, flip_y: bool) -> Self {
        Self { flip_x, flip_y }
    }
}

fn propagate_sprite_flips(
    parent_qry: Query<(Entity, &Children), With<Flippable>>,
    mut children_qry: Query<&mut Transform, (With<Parent>, With<Flippable>)>,
    mut flippable_qry: Query<&mut Flippable>,
) {
    for (parent_id, children) in parent_qry.iter() {
        for &child_id in children.iter() {
            let Ok(mut child_transform) = children_qry.get_mut(child_id) else {
                continue;
            };
            let Ok([parent_flippable, mut child_flippable]) =
                flippable_qry.get_many_mut([parent_id, child_id])
            else {
                continue;
            };

            if child_flippable.flip_x != parent_flippable.flip_x {
                child_transform.translation.x *= -1.;
                child_flippable.flip_x = parent_flippable.flip_x;
            }
            if child_flippable.flip_y != parent_flippable.flip_y {
                child_transform.translation.y *= -1.;
                child_flippable.flip_y = parent_flippable.flip_y;
            }
        }
    }
}

fn convert_flippables_to_sprite_flips(
    mut sprite_qry: Query<(&mut Sprite, &Flippable)>,
    mut tex_atlas_sprite_qry: Query<(&mut TextureAtlasSprite, &Flippable)>,
) {
    for (mut sprite, flippable) in sprite_qry.iter_mut() {
        sprite.flip_x = flippable.flip_x;
        sprite.flip_y = flippable.flip_y;
    }
    for (mut sprite, flippable) in tex_atlas_sprite_qry.iter_mut() {
        sprite.flip_x = flippable.flip_x;
        sprite.flip_y = flippable.flip_y;
    }
}
