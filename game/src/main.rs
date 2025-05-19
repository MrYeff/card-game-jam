#![allow(unused)]

use std::cmp::{max, min};

use bevy::{picking::hover::HoverMap, prelude::*, state::commands, transform::helper};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(SpritePickingSettings {
            require_markers: true,
            picking_mode: SpritePickingMode::BoundingBox,
        })
        .add_observer(DrawCardsEvent::handle)
        .add_systems(PostUpdate, check_game_won)
        .add_observer(check_game_lost)
        .run();
}

#[derive(Component)]
#[require(CardFace = panic!("CardFace Required") as CardFace, CardValue = panic!("CardValue Required") as CardValue)]
struct Card;

#[derive(Component)]
#[component(immutable)]
struct CardValue(u32);

#[derive(Component, Clone, Copy)]
#[component(immutable)]
enum CardFace {
    Weapon,
    Heal,
    Enemy,
}

#[derive(Component)]
#[relationship_target(relationship = PartOfStack)]
struct CardStack(Vec<Entity>);

#[derive(Component)]
#[relationship(relationship_target = CardStack)]
struct PartOfStack(Entity);

struct CardData {
    val: u32,
    face: CardFace,
}

type CardBundle = (Card, CardValue, CardFace);
impl From<CardData> for CardBundle {
    fn from(val: CardData) -> Self {
        (Card, CardValue(val.val), val.face)
    }
}

#[derive(Component, Default)]
struct CardStream(Vec<CardData>);

#[derive(Component)]
#[require(CardStream)]
struct DrawPile;

#[derive(Component, Default)]
#[require(Pickable)]
struct CardSlot;

#[derive(Component)]
#[require(CardSlot)]
struct RoomSlot;

#[derive(Component)]
#[require(CardSlot)]
struct PlayerWeaponSlot;

#[derive(Event)]
struct DrawCardsEvent;

impl DrawCardsEvent {
    fn handle(
        tr: Trigger<Self>,
        mut draw_pile: Query<&mut CardStream, With<DrawPile>>,
        room_slots: Query<Entity, (With<RoomSlot>, Without<Card>)>,
        mut commands: Commands,
    ) {
        let mut draw_pile = draw_pile.single_mut().expect("DrawPile not proper");

        for slot in room_slots {
            let Some(card) = draw_pile.0.pop() else {
                break;
            };

            commands.entity(slot).insert(CardBundle::from(card));
        }
    }
}

#[derive(Event)]
struct GameEndEvent {
    is_won: bool,
}

/// game is won if DrawPile and all RoomSlot are empty
fn check_game_won(
    draw_pile: Query<&CardStream, With<DrawPile>>,
    room_slots: Query<(), (With<RoomSlot>, With<Card>)>,
    mut commands: Commands,
) {
    let draw_pile = draw_pile.single().unwrap();

    if draw_pile.0.is_empty() && room_slots.is_empty() {
        commands.trigger(GameEndEvent { is_won: true });
    }
}

#[derive(Component)]
#[require(MaxHealth(20), Alive{ health: 20 }, DragTarget::PlayerBody, DropTarget::PlayerBody)]
struct Player;

#[derive(Component)]
struct Alive {
    health: u32,
}

#[derive(Component)]
struct Dead;

#[derive(Component)]
struct MaxHealth(u32);

#[derive(Event)]
struct AdjustHealthEvent(i32);

impl AdjustHealthEvent {
    fn handle(
        tr: Trigger<Self>,
        mut health: Query<(&mut Alive, &MaxHealth)>,
        mut commands: Commands,
    ) {
        let (mut hp, max_hp) = health.get_mut(tr.target()).unwrap();

        let hpi = hp.health as i32;
        hp.health = max(0, min(hpi + tr.0, max_hp.0 as i32)) as u32;

        if hp.health == 0 {
            commands.entity(tr.target()).remove::<Alive>().insert(Dead);
        }
    }
}

fn check_game_lost(
    tr: Trigger<OnAdd, Dead>,
    player: Query<(), With<Player>>,
    mut commands: Commands,
) {
    if player.contains(tr.target()) {
        commands.trigger(GameEndEvent { is_won: false });
    }
}

#[derive(Component)]
enum DragTarget {
    PlayerBody,
    PlayerWeapon,
    Weapon,
    Heal,
}

#[derive(Component)]
enum DropTarget {
    PlayerBody,
    PlayerWeapon,
    Weapon,
    Heal,
    Enemy,
}

#[derive(Event)]
struct DragDropEvent {
    drag_target: DragTarget,
    drop_target: DropTarget,
    drag_entity: Entity,
    drop_entity: Entity,
}

impl DragDropEvent {
    fn handle_equip_weapon(
        tr: Trigger<Self>,
        player_weapon: Query<Entity, With<PlayerWeaponSlot>>,
        mut commands: Commands,
    ) {
        let slot_id = match (&tr.drag_target, &tr.drop_target) {
            (DragTarget::PlayerBody, DropTarget::Weapon) => tr.drop_entity,
            (DragTarget::Weapon, DropTarget::PlayerBody)
            | (DragTarget::Weapon, DropTarget::PlayerWeapon) => tr.drag_entity,
            _ => {
                return;
            }
        };

        let player_weapon = player_weapon.single().unwrap();

        commands
            .entity(slot_id)
            .clone_with(player_weapon, |builder| {
                builder
                    .move_components(true)
                    .deny_all()
                    .allow::<CardBundle>();
            });
    }

    fn handle_heal_player(
        tr: Trigger<Self>,
        slots: Query<&CardValue, With<RoomSlot>>,
        mut commands: Commands,
    ) {
        let slot_id = match (&tr.drag_target, &tr.drop_target) {
            (DragTarget::PlayerBody, DropTarget::Heal) => tr.drop_entity,
            (DragTarget::Heal, DropTarget::PlayerBody) => tr.drag_entity,
            _ => {
                return;
            }
        };

        let heal = slots.get(slot_id).unwrap();

        commands.entity(slot_id).remove::<CardBundle>();
        commands.trigger(AdjustHealthEvent(heal.0 as i32));
    }

    fn handle_engage(
        tr: Trigger<Self>,
        slots: Query<&CardValue, With<RoomSlot>>,
        player_weapon: Query<(Entity, &CardValue, Option<&CardStack>), With<PlayerWeaponSlot>>,
        stacked_cards: Query<&CardValue, With<PartOfStack>>,
        mut commands: Commands,
    ) {
        if !matches!(
            (&tr.drag_target, &tr.drop_target),
            (DragTarget::PlayerBody, DropTarget::Enemy)
                | (DragTarget::PlayerWeapon, DropTarget::Enemy)
        ) {
            return;
        }

        let enemy_tier = slots.get(tr.drop_entity).unwrap().0;
        let damage_red = if matches!(&tr.drag_target, DragTarget::PlayerWeapon) {
            let (player_weapon, weapon_val, stack) = player_weapon.single().unwrap();
            if stack.is_some_and(|s| {
                s.0.iter()
                    .last()
                    .is_some_and(|c| stacked_cards.get(*c).unwrap().0 < enemy_tier)
            }) {
                return; // can't stack this enemy on this weapon
            } else {
                commands
                    .entity(tr.drop_entity)
                    .clone_and_spawn_with(|builder| {
                        builder
                            .move_components(true)
                            .deny_all()
                            .allow::<CardBundle>();
                    })
                    .insert((ChildOf(player_weapon), PartOfStack(player_weapon)));

                weapon_val.0
            }
        } else {
            commands.entity(tr.drop_entity).remove::<CardBundle>();
            0
        };

        commands.trigger(AdjustHealthEvent(min(
            0,
            damage_red as i32 - enemy_tier as i32,
        )));
    }
}

#[derive(Component)]
#[require(Pickable)]
struct EscapeDoor;

#[derive(Component)]
struct CanClick;

#[derive(Component)]
struct CanDrag;
