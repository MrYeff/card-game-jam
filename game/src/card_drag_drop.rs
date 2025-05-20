use bevy::prelude::*;

use crate::{
    card::Card,
    card_filter::CardFilter,
    card_slot::{CardSlot, PlacedOnSlot, PlacementOfCard},
};

/// marks if a card can be moved
#[derive(Component)]
struct Locked;

#[derive(Component)]
struct DragStartPoint(Vec3);

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
    cards: Query<&Transform, (With<Card>, Without<Locked>)>,
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

fn handle_drag(
    tr: Trigger<Pointer<Drag>>,
    mut tf: Query<&mut Transform, (With<Card>, Without<Locked>)>,
) {
    let Ok(mut tf) = tf.get_mut(tr.target()) else {
        return;
    };
    tf.translation += vec3(tr.delta.x, -tr.delta.y, 0.0);
}

fn handle_drag_drop(
    tr: Trigger<Pointer<DragDrop>>,
    cards: Query<&Card, Without<Locked>>,
    slots: Query<Option<&CardFilter>, (With<CardSlot>, Without<PlacementOfCard>)>,
    mut commands: Commands,
) {
    let entity = tr.dropped;
    let Ok(card) = cards.get(entity) else {
        return;
    };

    let slot = tr.target();
    let Ok(cf) = slots.get(slot) else {
        return;
    };

    if cf.is_some_and(|cf| !cf.check(card)) {
        return;
    }

    commands
        .entity(entity)
        .remove::<DragStartPoint>()
        .insert(PlacedOnSlot(slot));
}

#[allow(clippy::type_complexity)]
fn handle_drag_end(
    tr: Trigger<Pointer<DragEnd>>,
    mut cards: Query<Option<(&mut Transform, &DragStartPoint)>, (With<Card>, Without<Locked>)>,
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
