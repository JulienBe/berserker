mod scale;
mod control;
mod resize;
mod read_background;
mod grid;

use bevy::prelude::{App, Commands, ResMut, Assets, ColorMaterial, Res, AssetServer, Camera2dBundle, SpriteBundle, IntoSystem, Transform, With, Sprite};
use bevy::DefaultPlugins;
use std::fs::File;
use scale::*;
use control::*;
use resize::*;
use grid::*;
use bevy::ecs::Query;

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
        .add_system(floor_level.system())
        .run();
}

fn floor_level(grid: Res<Grid>, scale: Res<Scale>, mut q: Query<(&mut Transform, &Sprite, &mut SubPixelPos), With<Player>>) {
    for (mut trans, sprite, mut spp) in q.iter_mut() {
        let x = scale.to_ig_coord_x(trans.translation.x);
        let floor_y = grid.bridge_top[x];
        trans.translation.y = scale.to_screen_coord(floor_y as f32);
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
    commands.insert_resource(read_background::read(File::open("assets/background_colliders.png").unwrap()));
}