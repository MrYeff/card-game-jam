mod card;

use bevy::{asset::AssetPath, prelude::*};
use card::all_cards;
use std::fmt::{Display, Formatter};

use crate::card::Card;

#[derive(Resource)]
pub struct AssetHandles {
    card_fronts: [[Handle<Image>; 4]; 13],
}

impl AssetHandles {
    pub fn get_card_image(&self, card: &Card) -> Handle<Image> {
        self.card_fronts[(card.rank() - 1) as usize][card.suit() as usize].clone_weak()
    }

    pub fn load(server: &AssetServer) -> Self {
        let mut new = AssetHandles {
            card_fronts: std::array::from_fn(|_| std::array::from_fn(|_| Handle::default())),
        };

        for card in all_cards() {
            new.card_fronts[(card.rank() - 1) as usize][card.suit() as usize] = server.load(&card);
        }

        new
    }
}
