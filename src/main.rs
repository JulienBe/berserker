use bevy::prelude::{App, Commands, ResMut, Assets, ColorMaterial, Res, AssetServer, Camera2dBundle, SpriteBundle, Color, Transform, Vec3, Vec2, IntoSystem, Mat4, GlobalTransform, Windows, Size, Handle};
use bevy::DefaultPlugins;
use bevy::sprite::{Sprite, SpriteResizeMode};
use bevy::render::camera::{CameraProjection, DepthCalculation, Camera, VisibleEntities, OrthographicProjection, WindowOrigin};
use bevy::ecs::Query;
use bevy::render::render_graph::base;
use bevy::render::texture::Texture;
use bevy::sprite::SpriteResizeMode::Automatic;

struct Background;

const W: f32 = 160.0;
const H: f32 = 144.0;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(size_scaling.system())
        .run();
}

fn size_scaling(windows: Res<Windows>, mut q: Query<(&mut Sprite, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    let scale_h = window.height() / H;
    let scale_w = window.width() / W;
    let scale = if scale_h > scale_w {
        scale_h
    } else {
        scale_w
    };
    for (mut trans) in q.iter_mut() {
        trans.scale.x = scale;
        trans.scale.y = scale;
    }
}

fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>, asset_server: Res<AssetServer>,) {
    let background_handle = asset_server.load("background.png");
    commands
        // https://github.com/bevyengine/bevy/issues/239 can't zoom in for now. Monitor https://discord.com/channels/691052431525675048/742884593551802431/789823348401242133
        .spawn(Camera2dBundle::default())
        // Background
        .spawn(SpriteBundle {
            material: materials.add(background_handle.into()),
            ..Default::default()
        }).with(Background).with(Resize { x: 160, y: 144 });
}