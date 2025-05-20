mod card;
mod card_slot;

use bevy::{asset::AssetPath, prelude::*};
use card::all_cards;
use card_slot::all_card_slots;
use std::fmt::{Display, Formatter};

use crate::{card::Card, card_slot::CardSlot};

#[derive(Resource)]
pub struct AssetHandles {
    card_fronts: [[Handle<Image>; 4]; 13],
    card_slots: [Handle<Image>; 3],
}

impl AssetHandles {
    pub fn load(server: &AssetServer) -> Self {
        let mut new = AssetHandles {
            card_fronts: Default::default(),
            card_slots: Default::default(),
        };

        for card in all_cards() {
            new.card_fronts[(card.rank() - 1) as usize][card.suit() as usize] = server.load(&card);
        }

        for card_slot in all_card_slots() {
            new.card_slots[card_slot as usize] = server.load(&card_slot);
        }

        new
    }

    pub fn get_card_image(&self, card: &Card) -> Handle<Image> {
        self.card_fronts[(card.rank() - 1) as usize][card.suit() as usize].clone_weak()
    }

    pub fn get_card_slot_image(&self, card_slot: CardSlot) -> Handle<Image> {
        self.card_slots[card_slot as usize].clone_weak()
    }
}
