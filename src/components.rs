use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct CameraRig {
    pub pitch_angle: f32,
}

#[derive(Component)]
pub struct FpsText;
