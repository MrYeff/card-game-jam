#![allow(unused)]

#[macro_use]
mod enforce_exists;

mod assets;
mod card;
mod card_drag_drop;
mod card_filter;
mod card_slot;
mod health;
mod sprite_repr;
mod status_bar;

use assets::AssetHandles;
use bevy::{
    color::palettes::css::{GRAY, GREEN, RED, WHITE},
    prelude::*,
    state::commands,
};
use card::{Card, CardSuit};
use card_drag_drop::CardDragDropPlugin;
use card_filter::CardFilter;
use card_slot::{CardSlotPlugin, CardSlotSprite, PlacementOfCard};
use health::{AdjustHealth, Health, HealthPlugin, MaxHealth};
use sprite_repr::SpriteReprPlugin;
use status_bar::{StatusBar, StatusBarBackground, StatusBarPlugin, StatusBarRef, StatusBarType};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            SpriteReprPlugin,
            CardDragDropPlugin,
            CardSlotPlugin,
            HealthPlugin,
            StatusBarPlugin::<Health>::default(),
        ))
        .insert_resource(SpritePickingSettings {
            picking_mode: SpritePickingMode::BoundingBox,
            ..Default::default()
        })
        .add_systems(PreStartup, load_assets)
        .add_systems(Startup, setup_test_scene)
        .add_systems(Update, loose_health)
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

fn setup_test_scene(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        HealthTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
        Name::new("player"),
        MaxHealth(20),
        Health::new(15),
        related!(
            StatusBarRef[(
                StatusBarType::<Health>::default(),
                Sprite {
                    color: RED.into(),
                    ..default()
                },
                StatusBar::new(Vec2::new(200.0, 40.0)),
                StatusBarBackground(Sprite {
                    color: GRAY.into(),
                    ..default()
                })
            )]
        ),
    ));
}

#[derive(Component, Default)]
struct HealthTimer(Timer);

fn loose_health(
    mut qs: Query<(&mut HealthTimer, Entity), With<Health>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (mut ht, e) in qs.iter_mut() {
        if ht.0.tick(time.delta()).just_finished() {
            commands.entity(e).trigger(AdjustHealth(-1));
        }
    }
}
