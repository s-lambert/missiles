mod level_plugin;
mod player_plugin;

use bevy::prelude::*;
use bevy_rapier3d::{prelude::*, render::RapierDebugRenderPlugin};
use level_plugin::LevelPlugin;
use player_plugin::PlayerPlugin;

#[derive(States, Default, Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum GameState {
    #[default]
    Battle,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Missiles!".to_string(),
                        resolution: (500.0, 500.0).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(RapierConfiguration {
            gravity: Vect::new(0.0, 0.0, 0.0),
            ..default()
        })
        .add_state::<GameState>()
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(PlayerPlugin)
        .add_plugin(LevelPlugin)
        .add_system(bevy::window::close_on_esc)
        .run();
}
