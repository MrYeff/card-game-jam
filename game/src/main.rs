#![allow(unused)]

mod assets;
mod card;
mod sprite_repr;

use std::{
    cmp::{max, min},
    result::{self, Result},
};

use assets::AssetHandles;
use bevy::{picking::hover::HoverMap, prelude::*, state::commands, transform::helper};
use card::{Card, CardSuit};
use sprite_repr::SpriteReprPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SpriteReprPlugin))
        .insert_resource(SpritePickingSettings {
            require_markers: true,
            ..Default::default()
        })
        .add_systems(PreStartup, load_assets)
        .add_systems(Startup, setup_scene)
        .run();
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(AssetHandles::load(&asset_server));
}

fn setup_scene(mut commands: Commands, assets: Res<AssetHandles>) {
    commands.spawn(Camera2d);

    commands.spawn(Card::new(CardSuit::Hearts, 13));
}
