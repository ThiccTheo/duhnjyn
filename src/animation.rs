use bevy::prelude::*;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (adjust_sprite_indices, animate_sprites).chain());
    }
}

#[derive(Component, PartialEq)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub fn adjust_sprite_indices(
    mut animation_qry: Query<
        (&mut TextureAtlasSprite, &AnimationIndices),
        Changed<AnimationIndices>,
    >,
) {
    for (mut tex_atlas_sprite, animation_indices) in animation_qry.iter_mut() {
        tex_atlas_sprite.index = animation_indices.first;
    }
}

fn animate_sprites(
    time: Res<Time>,
    mut animation_qry: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (animation_indices, mut animation_timer, mut tex_atlas_sprite) in animation_qry.iter_mut() {
        animation_timer.tick(time.delta());
        if animation_timer.just_finished() {
            tex_atlas_sprite.index = if tex_atlas_sprite.index == animation_indices.last {
                animation_indices.first
            } else {
                tex_atlas_sprite.index + 1
            }
        }
    }
}
