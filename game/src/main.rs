#![allow(unused)]

mod assets;
mod card;
mod card_drag_drop;
mod card_filter;
mod card_slot;
mod sprite_repr;

use assets::AssetHandles;
use bevy::prelude::*;
use card::{Card, CardSuit};
use card_drag_drop::CardDragDropPlugin;
use card_filter::CardFilter;
use card_slot::{CardSlotSprite, CardSlotPlugin, PlacementOfCard};
use sprite_repr::SpriteReprPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            SpriteReprPlugin,
            CardDragDropPlugin,
            CardSlotPlugin,
        ))
        .insert_resource(SpritePickingSettings {
            picking_mode: SpritePickingMode::BoundingBox,
            ..Default::default()
        })
        .add_systems(PreStartup, load_assets)
        .add_systems(Startup, setup_scene)
        .run();
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(AssetHandles::load(&asset_server));
}

fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Cards
    commands.spawn((
        CardSlotSprite::Empty,
        Pickable::default(),
        Transform::from_xyz(-300.0, 200.0, 0.0),
        related![PlacementOfCard[(Card::new(CardSuit::Hearts, 11), Pickable::default(),)]],
    ));
    commands.spawn((
        CardSlotSprite::Empty,
        Pickable::default(),
        Transform::from_xyz(-100.0, 200.0, 0.0),
        related![PlacementOfCard[(Card::new(CardSuit::Spades, 10), Pickable::default(),)]],
    ));
    commands.spawn((
        CardSlotSprite::Empty,
        Pickable::default(),
        Transform::from_xyz(100.0, 200.0, 0.0),
        related![PlacementOfCard[(Card::new(CardSuit::Diamonds, 13), Pickable::default(),)]],
    ));
    commands.spawn((
        CardSlotSprite::Empty,
        Pickable::default(),
        Transform::from_xyz(300.0, 200.0, 0.0),
        related![PlacementOfCard[(Card::new(CardSuit::Clubs, 2), Pickable::default(),)]],
    ));

    // Card Slots
    commands.spawn((
        CardSlotSprite::Body,
        Pickable::default(),
        Transform::from_xyz(-100.0, -150.0, 0.0),
        CardFilter::empty().with_suit([CardSuit::Hearts]),
    ));

    commands.spawn((
        CardSlotSprite::Weapon,
        Pickable::default(),
        Transform::from_xyz(100.0, -150.0, 0.0),
        CardFilter::empty().with_suit([CardSuit::Diamonds]),
    ));
}
