mod scale;
mod control;
mod resize;

use bevy::prelude::{App, Commands, ResMut, Assets, ColorMaterial, Res, AssetServer, Camera2dBundle, SpriteBundle, IntoSystem};
use bevy::DefaultPlugins;
use scale::*;
use control::*;
use resize::*;

struct Background;
struct Player;
struct SubPixelPos { x: f32, y: f32 }

impl Default for SubPixelPos {
    fn default() -> Self {
        Self {
            x: 0.0, y: 0.0,
        }
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(resize_notificator.system())
        .add_system(move_player.system())
        .run();
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