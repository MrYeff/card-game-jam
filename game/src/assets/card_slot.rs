use bevy::asset::AssetPath;
use enum_iterator::all;

use crate::card_slot::CardSlotSprite;

impl From<&CardSlotSprite> for AssetPath<'_> {
    fn from(slot: &CardSlotSprite) -> Self {
        let s = match slot {
            CardSlotSprite::Empty => "empty",
            CardSlotSprite::Weapon => "weapon",
            CardSlotSprite::Body => "body",
            CardSlotSprite::Enemy => "enemy",
        };

        AssetPath::from(format!("slots/{}.png", s))
    }
}

pub(super) fn all_card_slots() -> impl Iterator<Item = CardSlotSprite> {
    all::<CardSlotSprite>()
}
