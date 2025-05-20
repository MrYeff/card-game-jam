use bevy::prelude::*;

use crate::{assets::AssetHandles, card::Card};

use super::SpriteRepr;

impl SpriteRepr for Card {
    fn to_sprite(&self, assets: &AssetHandles) -> Sprite {
        Sprite::from_image(assets.get_card_image(self))
    }
}
