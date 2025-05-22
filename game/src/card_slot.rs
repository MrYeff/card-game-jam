use bevy::prelude::*;
use enum_iterator::Sequence;

#[derive(Component, Default)]
pub struct CardSlot;

#[derive(Component, Sequence, Clone, Copy)]
#[require(CardSlot)]
#[repr(usize)]
pub enum CardSlotSprite {
    Empty,
    Weapon,
    Body,
}

#[derive(Component)]
#[relationship(relationship_target=PlacementOfCard)]
pub struct PlacedOnSlot(pub Entity);

#[derive(Component)]
#[relationship_target(relationship=PlacedOnSlot)]
pub struct PlacementOfCard(Entity);

impl PlacementOfCard {
    pub fn get(&self) -> Entity {
        self.0
    }
}

pub struct CardSlotPlugin;

#[derive(Event)]
pub struct RecievedCard(pub Entity);

impl Plugin for CardSlotPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(handle_placed_on_added)
            .add_observer(handle_placed_on_removed);
    }
}

fn handle_placed_on_added(
    tr: Trigger<OnInsert, PlacedOnSlot>,
    placed_on: Query<&PlacedOnSlot>,
    mut commands: Commands,
) {
    let card = tr.target();
    let slot = placed_on.get(card).unwrap().0;
    commands
        .entity(card)
        .insert((ChildOf(slot), Transform::from_xyz(0.0, 0.0, 1.0)));

    commands.entity(slot).trigger(RecievedCard(card));
}

fn handle_placed_on_removed(tr: Trigger<OnRemove, PlacedOnSlot>, mut commands: Commands) {
    let entity = tr.target();
    if let Ok(mut ec) = commands.get_entity(entity) {
        ec.try_remove::<ChildOf>();
    }
}
