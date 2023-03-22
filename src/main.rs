use bevy::prelude::*;

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
        .add_system(bevy::window::close_on_esc)
        .add_state::<GameState>()
        .run();
}
