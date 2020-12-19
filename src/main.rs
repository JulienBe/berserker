use bevy::prelude::{App, Commands, ResMut, Assets, ColorMaterial, Res, AssetServer, Camera2dBundle, SpriteBundle, Color, Transform, Vec3, Vec2, IntoSystem, Mat4, GlobalTransform};
use bevy::DefaultPlugins;
use bevy::sprite::Sprite;
use bevy::render::camera::{CameraProjection, DepthCalculation, Camera, VisibleEntities};

struct Background;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>, asset_server: Res<AssetServer>,) {

    let background_handle = asset_server.load("background.png");

    commands
        .spawn(Camera2dBundle::default())
        // Background
        .spawn(SpriteBundle {
            material: materials.add(background_handle.into()),
            ..Default::default()
        }).with(Background);
}