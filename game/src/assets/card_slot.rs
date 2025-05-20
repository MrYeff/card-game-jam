use bevy::asset::AssetPath;
use enum_iterator::all;

use crate::card_slot::CardSlot;

impl From<&CardSlot> for AssetPath<'_> {
    fn from(slot: &CardSlot) -> Self {
        let s = match slot {
            CardSlot::Empty => "empty",
            CardSlot::Weapon => "weapon",
            CardSlot::Body => "body",
        };

        AssetPath::from(format!("slots/{}.png", s))
    }
}

pub(super) fn all_card_slots() -> impl Iterator<Item = CardSlot> {
    all::<CardSlot>()
}
