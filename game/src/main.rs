#![allow(unused)]

#[macro_use]
mod enforce_exists;

mod assets;
mod card;
mod card_drag_drop;
mod card_filter;
mod card_slot;
mod despawn;
mod health;
mod sprite_repr;
mod status_bar;

use assets::AssetHandles;
use bevy::{
    color::palettes::css::{GRAY, RED},
    prelude::*,
    remote::{RemotePlugin, http::RemoteHttpPlugin},
    sprite::Anchor,
};
use card::{Card, CardSuit};
use card_drag_drop::{CardDragDropPlugin, Locked};
use card_filter::CardFilter;
use card_slot::{
    CardSlotPlugin, CardSlotSprite, PlacedOnSlot, PlacementOfCard, RecievedCard, ShowWhenOcupied,
};
use despawn::{DespawnDelayed, DespawnPlugin};
use health::{AdjustHealth, Health, HealthPlugin, MaxHealth};
use sprite_repr::SpriteReprPlugin;
use status_bar::{StatusBarOf, StatusBarPlugin, StatusBarType};
fn main() {
    App::new()
        .add_plugins((
            RemotePlugin::default(),
            RemoteHttpPlugin::default().with_port(15702),
        ))
        .add_plugins((
            DefaultPlugins,
            SpriteReprPlugin,
            CardDragDropPlugin,
            CardSlotPlugin,
            HealthPlugin,
            StatusBarPlugin::<Health>::default(),
            DespawnPlugin::<PostUpdate>::default(),
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

    let player = commands.spawn((Player, MaxHealth(20), Health(20))).id();

    // Cards
    commands.spawn((
        CardSlotSprite::Empty,
        ShowWhenOcupied,
        Pickable::default(),
        Transform::from_xyz(-300.0, 200.0, 0.0),
        related![
            PlacementOfCard[(
                Name::new("Card H11"),
                Card::new(CardSuit::Hearts, 11),
                Pickable::default(),
            )]
        ],
    ));
    commands.spawn((
        CardSlotSprite::Empty,
        ShowWhenOcupied,
        Pickable::default(),
        Transform::from_xyz(-100.0, 200.0, 0.0),
        related![
            PlacementOfCard[(
                Name::new("Card D10"),
                Card::new(CardSuit::Diamonds, 10),
                Pickable::default(),
            )]
        ],
    ));
    commands.spawn((
        CardSlotSprite::Empty,
        ShowWhenOcupied,
        Pickable::default(),
        Transform::from_xyz(100.0, 200.0, 0.0),
        related![
            PlacementOfCard[(
                Name::new("Card D13"),
                Card::new(CardSuit::Diamonds, 13),
                Pickable::default(),
            )]
        ],
    ));
    commands.spawn((
        CardSlotSprite::Empty,
        ShowWhenOcupied,
        Pickable::default(),
        Transform::from_xyz(300.0, 200.0, 0.0),
        related![
            PlacementOfCard[(
                Name::new("Card C10"),
                Card::new(CardSuit::Clubs, 10),
                Pickable::default(),
            )]
        ],
    ));

    // Card Slots
    let body = commands
        .spawn((
            Name::new("Body"),
            CardSlotSprite::Body,
            ShowWhenOcupied,
            Pickable::default(),
        ))
        .observe(handle_card_on_body)
        .id();

    commands
        .spawn((
            Transform::from_xyz(-100.0, -150.0, 0.0),
            InheritedVisibility::default(),
            children![(
                Transform::from_xyz(-72.5, -130.0, 0.0),
                InheritedVisibility::default(),
                children![
                    (
                        StatusBarOf(player),
                        StatusBarType::<Health>::default(),
                        Sprite {
                            color: RED.into(),
                            custom_size: Some(Vec2::new(145.0, 35.0)),
                            anchor: Anchor::CenterLeft,
                            ..default()
                        }
                    ),
                    (
                        Transform::from_xyz(0.0, 0.0, -1.0),
                        Sprite {
                            color: GRAY.into(),
                            custom_size: Some(Vec2::new(145.0, 35.0)),
                            anchor: Anchor::CenterLeft,
                            ..default()
                        }
                    )
                ]
            )],
        ))
        .add_child(body);

    let weapon = commands
        .spawn((
            Name::new("Weapon"),
            CardSlotSprite::Weapon,
            ShowWhenOcupied,
            Pickable::default(),
            Transform::from_xyz(100.0, -150.0, 0.0),
            CardFilter::empty().with_suit([CardSuit::Diamonds]),
        ))
        .observe(handle_card_on_weapon)
        .id();

    commands
        .entity(player)
        .insert((PlayerBody(body), PlayerWeapon(weapon)));
}

#[derive(Component)]
#[relationship(relationship_target=PlayerWeaponOf)]
struct PlayerWeapon(Entity);

#[derive(Component)]
#[relationship_target(relationship=PlayerWeapon)]
struct PlayerWeaponOf(Entity);

#[derive(Component)]
#[relationship(relationship_target=PlayerBodyOf)]
struct PlayerBody(Entity);

#[derive(Component)]
#[relationship_target(relationship=PlayerBody)]
struct PlayerBodyOf(Entity);

#[derive(Component)]
#[require(Name = Name::new("Player"))]
struct Player;

fn handle_card_on_body(
    tr: Trigger<RecievedCard>,
    card_slots: Query<&PlacementOfCard>,
    cards: Query<&Card, With<PlacedOnSlot>>,
    player: Query<(Entity, &PlayerWeapon), With<Player>>,
    mut commands: Commands,
) {
    let card_e = tr.0;
    let card = cards.get(card_e).unwrap();
    let (player_e, weapon) = player.single().unwrap();

    match card.suit() {
        CardSuit::Hearts => {
            commands
                .entity(player_e)
                .trigger(AdjustHealth(card.rank() as i32));

            commands.entity(card_e).trigger(DespawnDelayed);
        }
        CardSuit::Diamonds => {
            if let Ok(weapon_slot) = card_slots.get(weapon.0) {
                commands
                    .entity(weapon_slot.get())
                    .remove::<PlacedOnSlot>()
                    .trigger(DespawnDelayed);
            }

            commands.entity(card_e).insert(PlacedOnSlot(weapon.0));
        }
        CardSuit::Clubs | CardSuit::Spades => {
            commands
                .entity(player_e)
                .trigger(AdjustHealth(-(card.rank() as i32)));

            commands.entity(card_e).trigger(DespawnDelayed);
        }
    }
}

type HandleEnemyOnEnemySystem = Box<
    dyn Fn(Trigger<RecievedCard>, Query<&Card, With<PlacedOnSlot>>, Commands)
        + Send
        + Sync
        + 'static,
>;

fn make_handle_enemy_on_enemy(card_rank: u32) -> HandleEnemyOnEnemySystem {
    Box::new(
        move |tr: Trigger<RecievedCard>,
              cards: Query<&Card, With<PlacedOnSlot>>,
              mut commands: Commands| {
            let card_e = tr.0;
            let card = cards.get(card_e).unwrap();

            commands
                .entity(card_e)
                .insert(Locked)
                .with_child((
                    CardSlotSprite::Enemy,
                    Pickable::default(),
                    Transform::from_xyz(0.0, -40.0, 1.0),
                    CardFilter::empty()
                        .with_suit([CardSuit::Clubs, CardSuit::Spades])
                        .with_max_rank(card_rank - 1),
                ))
                .observe(make_handle_enemy_on_enemy(card.rank() - 1));
        },
    )
}

fn handle_card_on_weapon(tr: Trigger<RecievedCard>, mut commands: Commands) {
    let card_e = tr.0;

    commands
        .entity(card_e)
        .insert(Locked)
        .with_child((
            CardSlotSprite::Enemy,
            Pickable::default(),
            Transform::from_xyz(0.0, -40.0, 1.0),
            CardFilter::empty().with_suit([CardSuit::Clubs, CardSuit::Spades]),
        ))
        .observe(handle_enemy_on_weapon);
}

fn handle_enemy_on_weapon(
    tr: Trigger<RecievedCard>,
    cards: Query<&Card, With<PlacedOnSlot>>,
    mut commands: Commands,
) {
    let card_e = tr.0;
    let card = cards.get(card_e).unwrap();

    commands
        .entity(card_e)
        .insert(Locked)
        .with_child((
            CardSlotSprite::Enemy,
            Pickable::default(),
            Transform::from_xyz(0.0, -40.0, 1.0),
            CardFilter::empty().with_suit([CardSuit::Clubs, CardSuit::Spades]),
        ))
        .observe(make_handle_enemy_on_enemy(card.rank()));
}
