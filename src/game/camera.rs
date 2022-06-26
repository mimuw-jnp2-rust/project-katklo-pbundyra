use bevy::prelude::*;
use bevy::render::camera::Camera2d;
use bevy::{
    math::Vec3,
    prelude::OrthographicCameraBundle,
    render::camera::{DepthCalculation, OrthographicProjection, ScalingMode},
};
use bevy_rapier2d::prelude::RigidBody;

use crate::game::Player;

pub fn new_camera_2d() -> OrthographicCameraBundle<Camera2d> {
    let far = 1000.0;
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection = OrthographicProjection {
        far,
        depth_calculation: DepthCalculation::ZDifference,
        scaling_mode: ScalingMode::FixedHorizontal,
        ..Default::default()
    };
    camera.transform.scale = Vec3::new(10., 10., 1.);
    camera
}

pub fn camera_follow_player(
    mut cameras: Query<&mut Transform, With<Camera>>,
    players: Query<(&Transform, &RigidBody, &Player), Without<Camera>>,
) {
    if let Ok((player, _, _)) = players.get_single() {
        for mut camera in cameras.iter_mut() {
            camera.translation.x = player.translation.x;
            camera.translation.y = player.translation.y;
        }
    }
}
