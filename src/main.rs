mod animation;
mod game_state;
mod main_camera;
mod mouse_position;
mod physics;
mod player;
mod sprite_flip;
mod tile;
mod weapon;

use {
    animation::AnimationPlugin,
    bevy::{
        // diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
        prelude::*,
        window::{PresentMode, WindowMode, WindowResolution},
    },
    bevy_ecs_tilemap::prelude::*,
    bevy_inspector_egui::quick::WorldInspectorPlugin,
    bevy_rapier2d::prelude::*,
    game_state::GameState,
    leafwing_input_manager::prelude::*,
    // main_camera::MainCameraPlugin,
    mouse_position::MousePositionPlugin,
    physics::PhysicsPlugin,
    player::{PlayerAction, PlayerPlugin},
    sprite_flip::SpriteFlipPlugin,
    tile::TilePlugin,
    weapon::WeaponPlugin,
};

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::AutoNoVsync,
                        mode: WindowMode::Windowed,
                        resolution: WindowResolution::new(1280., 720.),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        title: String::from("Duhnjyn"),
                        ..default()
                    }),
                    ..default()
                }),
            // LogDiagnosticsPlugin::default(),
            // FrameTimeDiagnosticsPlugin::default(),
            WorldInspectorPlugin::new(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            InputManagerPlugin::<PlayerAction>::default(),
            TilemapPlugin,
            // MainCameraPlugin,
            PlayerPlugin,
            TilePlugin,
            PhysicsPlugin,
            SpriteFlipPlugin,
            AnimationPlugin,
            MousePositionPlugin,
            // WeaponPlugin,
        ))
        .run();
}
