use bevy::{math::VectorSpace, prelude::*};

use crate::{assets::AssetHandles, card::Card, card_slot::CardSlot};

use super::SpriteRepr;

impl SpriteRepr for CardSlot {
    fn to_sprite(&self, assets: &AssetHandles) -> Sprite {
        Sprite {
            image: assets.get_card_slot_image(*self),
            custom_size: Some(Vec2::new(145.0, 200.0)),
            ..Default::default()
        }
    }
}
