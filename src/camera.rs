use bevy::prelude::*;

use crate::graphics::TILE_SIZE;

pub fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.transform.translation = Vec3::new(
        20. * TILE_SIZE,
        10. * TILE_SIZE,
        camera.transform.translation.z,
    );
    camera.transform.scale = camera.transform.scale * Vec3::splat(2.);
    commands.spawn(camera);
}
