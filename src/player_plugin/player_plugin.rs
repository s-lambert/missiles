use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::GameState;

pub struct PlayerPlugin;

#[derive(Component)]
struct Player;

fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, -10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    commands.spawn((
        Player,
        MaterialMeshBundle {
            mesh: meshes.add(shape::UVSphere::default().into()),
            material: materials.add(Color::AZURE.into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Velocity::default(),
        Collider::ball(1.0),
        ExternalForce::default(),
        Damping {
            linear_damping: 0.0,
            angular_damping: 0.0,
        },
    ));
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_player.in_schedule(OnEnter(GameState::Battle)));
    }
}
