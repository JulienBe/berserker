use bevy::prelude::{App, Commands, ResMut, Assets, ColorMaterial, Res, AssetServer, Camera2dBundle, SpriteBundle, Color, Transform, Vec3, Vec2, IntoSystem, Mat4, GlobalTransform, Windows, Size, Handle, Input, KeyCode};
use bevy::DefaultPlugins;
use bevy::sprite::{Sprite, SpriteResizeMode};
use bevy::render::camera::{CameraProjection, DepthCalculation, Camera, VisibleEntities, OrthographicProjection, WindowOrigin};
use bevy::ecs::{Query, With};
use bevy::render::render_graph::base;
use bevy::render::texture::Texture;
use bevy::sprite::SpriteResizeMode::Automatic;
use bevy::app::Events;
use bevy::window::WindowResized;

struct Background;
struct Player;
struct SubPixelPos { x: f32, y: f32 }
struct Scale { h: f32, w: f32, g: f32 }

impl Default for SubPixelPos {
    fn default() -> Self {
        Self {
            x: 0.0, y: 0.0,
        }
    }
}
// remove when camera issue fixed
impl Default for Scale {
    fn default() -> Self {
        Self {
            h: 0.0, w: 0.0, g: 0.0,
        }
    }
}

impl Scale {
    fn update(&mut self, window_w: f32, window_h: f32) {
        self.w = window_w / W;
        self.h = window_h / H;
        self.g = if self.h > self.w {
            self.h
        } else {
            self.w
        };
    }
    fn translate_pos(&self, spp: &SubPixelPos, sprite: &Sprite) -> (f32, f32) {
        let mut pos = (spp.x * self.g, spp.y * self.g);
        // align to scale
        pos.0 -= pos.0 % self.g;
        pos.1 -= pos.1 % self.g;
        // align to other sprites
        let w_mod = (sprite.size.x % 2.0) * (self.g / 2.0);
        let h_mod = (sprite.size.y % 2.0) * (self.g / 2.0);
        pos.0 += w_mod;
        pos.1 += h_mod;
        pos
    }
}

const W: f32 = 160.0;
const H: f32 = 144.0;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(resize_notificator.system())
        .add_system(move_player.system())
        .run();
}

fn resize_notificator(resize_event: Res<Events<WindowResized>>, mut scale: ResMut<Scale>, mut q: Query<(&mut Transform), With<Sprite>>) {
    let mut reader = resize_event.get_reader();
    for e in reader.iter(&resize_event) {
        scale.update(e.width, e.height)
    }
    for mut trans in q.iter_mut() {
        trans.scale.x = scale.g;
        trans.scale.y = scale.g;
        trans.translation.x = 0.0;
        trans.translation.y = 0.0;
    }
}

fn move_player(keys: Res<Input<KeyCode>>, scale: Res<Scale>, mut q: Query<(&mut Transform, &mut SubPixelPos, &Sprite), With<Player>>) {
    let mut dir = Vec2::new(0.0, 0.0);
    if keys.pressed(KeyCode::Left) {
        dir.x -= 1.0;
    }
    if keys.pressed(KeyCode::Right) {
        dir.x += 1.0;
    }
    if keys.pressed(KeyCode::Down) {
        dir.y -= 1.0;
    }
    if keys.pressed(KeyCode::Up) {
        dir.y += 1.0;
    }
    for (mut trans, mut spp, sprite) in q.iter_mut() {
        spp.x += dir.x;
        spp.y += dir.y;
        let pos = scale.translate_pos(&spp, &sprite);
        trans.translation.x = pos.0;
        trans.translation.y = pos.1;
    }
}

fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>, asset_server: Res<AssetServer>,) {
    let background_handle = asset_server.load("background.png");
    let player = asset_server.load("player.png");
    commands
        // https://github.com/bevyengine/bevy/issues/239 can't zoom in for now. Monitor https://discord.com/channels/691052431525675048/742884593551802431/789823348401242133
        .spawn(Camera2dBundle::default())
        // Background
        .spawn(SpriteBundle {
            material: materials.add(background_handle.into()),
            ..Default::default()
        }).with(Background)
        // Player
        .spawn(SpriteBundle {
            material: materials.add(player.into()),
            ..Default::default()
        }).with(Player).with(SubPixelPos::default());
    commands.insert_resource(Scale::default());

}