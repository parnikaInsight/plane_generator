use bevy::prelude::*;

// set up a simple 3D scene
pub fn setup_camera(
    mut commands: Commands,
) {
    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle { 
        transform: Transform::from_xyz(0.0, 7.5, 0.5).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
    