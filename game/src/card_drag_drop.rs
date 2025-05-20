use bevy::{prelude::*, state::commands};

use crate::{
    card::Card,
    card_slot::{CardSlot, PlacedOnSlot, PlacementOfCard},
};

/// marks if a card can be moved
#[derive(Component)]
struct Locked;

#[derive(Component)]
struct DragStartPoint(Vec3);

type DomainQF = (With<Card>, Without<Locked>);

pub struct CardDragDropPlugin;

impl Plugin for CardDragDropPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(handle_drag_start)
            .add_observer(handle_drag)
            .add_observer(handle_drag_drop)
            .add_observer(handle_drag_end);
    }
}

fn handle_drag_start(
    tr: Trigger<Pointer<DragStart>>,
    cards: Query<&Transform, DomainQF>,
    mut commands: Commands,
) {
    let entity = tr.target();
    let Ok(trf) = cards.get(entity) else {
        return;
    };
    commands
        .entity(entity)
        .remove::<Pickable>()
        .insert(DragStartPoint(trf.translation));
}

fn handle_drag(tr: Trigger<Pointer<Drag>>, mut tf: Query<&mut Transform, DomainQF>) {
    let Ok(mut tf) = tf.get_mut(tr.target()) else {
        return;
    };
    tf.translation += vec3(tr.delta.x, -tr.delta.y, 0.0);
}

fn handle_drag_drop(
    tr: Trigger<Pointer<DragDrop>>,
    mut cards: Query<(), DomainQF>,
    slots: Query<(), (With<CardSlot>, Without<PlacementOfCard>)>,
    mut commands: Commands,
) {
    let card = tr.dropped;
    if !cards.contains(card) {
        return;
    }

    let slot = tr.target();
    if !slots.contains(slot) {
        return;
    }

    commands
        .entity(card)
        .remove::<DragStartPoint>()
        .insert(PlacedOnSlot(slot));
}

fn handle_drag_end(
    tr: Trigger<Pointer<DragEnd>>,
    mut cards: Query<Option<(&mut Transform, &DragStartPoint)>, DomainQF>,
    mut commands: Commands,
) {
    let Ok(card) = cards.get_mut(tr.target()) else {
        return;
    };

    commands.entity(tr.target()).insert(Pickable::default());

    let Some((mut trf, sp)) = card else {
        return;
    };

    commands.entity(tr.target()).remove::<DragStartPoint>();
    trf.translation = sp.0
}
