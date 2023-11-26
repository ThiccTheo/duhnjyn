mod game_state;
mod main_camera;
mod physics;
mod player;
mod tile;

use {
    bevy::prelude::*,
    bevy_inspector_egui::quick::WorldInspectorPlugin,
    bevy_rapier2d::prelude::*,
    game_state::GameState,
    leafwing_input_manager::prelude::*,
    main_camera::MainCameraPlugin,
    physics::PhysicsPlugin,
    player::{PlayerAction, PlayerPlugin},
    tile::TilePlugin,
};

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            WorldInspectorPlugin::new(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            InputManagerPlugin::<PlayerAction>::default(),
            MainCameraPlugin,
            PlayerPlugin,
            TilePlugin,
            PhysicsPlugin,
        ))
        .run();
}
