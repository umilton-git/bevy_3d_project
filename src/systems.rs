use crate::components::*;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy_ecs::prelude::Res;

/// set up a simple 3D scene

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(50.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 15000.0,
            shadows_enabled: true,
            range: 150.0,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(CameraRig { pitch_angle: 0.0 });
}

// Allows movement; WASD for directional movement, Q and E to move up and down

pub fn movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&CameraRig, &mut Transform)>,
    time: Res<Time>,
) {
    let mut direction = Vec3::ZERO;
    let mut global_direction = Vec3::ZERO;

    let move_speed = 10.0; // Adjust as necessary
    let delta_time = time.delta_seconds();

    if keyboard_input.pressed(KeyCode::S) {
        direction += Vec3::Z;
    }

    if keyboard_input.pressed(KeyCode::W) {
        direction -= Vec3::Z;
    }

    if keyboard_input.pressed(KeyCode::A) {
        direction -= Vec3::X;
    }

    if keyboard_input.pressed(KeyCode::D) {
        direction += Vec3::X;
    }

    if keyboard_input.pressed(KeyCode::Q) {
        global_direction -= Vec3::Y;
    }

    if keyboard_input.pressed(KeyCode::E) {
        global_direction += Vec3::Y;
    }

    for (_rig, mut transform) in query.iter_mut() {
        let forward = transform.rotation * Vec3::Z;
        let right = transform.rotation * Vec3::X;
        if keyboard_input.pressed(KeyCode::LShift) || keyboard_input.pressed(KeyCode::RShift) {
            transform.translation +=
                (forward * direction.z + right * direction.x) * move_speed * 2.0 * delta_time;
        } else {
            transform.translation +=
                (forward * direction.z + right * direction.x) * move_speed * delta_time;
        }

        // Apply global up and down movement
        transform.translation += global_direction * move_speed * delta_time;
    }
}

// Allows camera movement

pub fn look_system(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&mut CameraRig, &mut Transform)>,
) {
    let mut delta: Vec2 = Vec2::ZERO;
    for event in mouse_motion_events.iter() {
        delta += event.delta;
    }

    for (mut rig, mut transform) in query.iter_mut() {
        let mouse_speed = 0.005; // Adjust as necessary

        // Yaw around global axis (y-axis)
        let global_yaw = Quat::from_rotation_y(-delta.x * mouse_speed);
        transform.rotation = global_yaw * transform.rotation;

        // Compute the desired pitch angle
        let desired_pitch_angle = (rig.pitch_angle - delta.y * mouse_speed).clamp(-1.0472, 1.0472);

        // Compute the difference in pitch from the previous frame
        let pitch_delta = desired_pitch_angle - rig.pitch_angle;

        // Apply the pitch rotation
        let local_pitch = Quat::from_rotation_x(pitch_delta);
        transform.rotation = transform.rotation * local_pitch;

        // Update the stored pitch angle
        rig.pitch_angle = desired_pitch_angle;
    }
}

// Cursor lock system

pub fn grab_mouse(
    mut windows: Query<&mut Window>,
    mouse: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let mut window = windows.single_mut();

    if mouse.just_pressed(MouseButton::Left) {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }
}
