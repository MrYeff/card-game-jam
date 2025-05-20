use bevy::prelude::*;

use crate::{assets::AssetHandles, card::Card};

use super::SpriteRepr;

impl SpriteRepr for Card {
    fn to_sprite(&self, assets: &AssetHandles) -> Sprite {
        Sprite {
            image: assets.get_card_image(self),
            custom_size: Some(Vec2::new(145.0, 200.0)),
            ..Default::default()
        }
    }
}
