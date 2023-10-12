use bevy::prelude::*;

use crate::config::SCALE;

pub struct StagePlugin;

#[derive(Resource, Default, Copy, Clone, Debug)]
pub struct StageFloor(pub f32);

impl Plugin for StagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
    }
}

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("sprites/ken_stage.png"),
        transform: Transform::from_xyz(0., 0., -1.).with_scale(Vec3::new(SCALE, SCALE, -1.)),
        ..default()
    });
}
