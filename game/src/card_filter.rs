use bevy::prelude::*;

use crate::card::{Card, CardSuit};

#[derive(Component, Default, Clone)]
pub struct CardFilter {
    suits: Vec<CardSuit>,
    /// included
    rank_min: Option<u32>,
    /// included
    rank_max: Option<u32>,
}

impl CardFilter {
    pub fn check(&self, card: &Card) -> bool {
        self.suits.contains(&card.suit())
            && self.rank_min.is_none_or(|min| min <= card.rank())
            && self.rank_max.is_none_or(|min| min >= card.rank())
    }

    pub fn empty() -> Self {
        Default::default()
    }

    pub fn with_suit(mut self, suits: impl IntoIterator<Item = CardSuit>) -> Self {
        self.suits = suits.into_iter().collect();
        self
    }

    pub fn with_min_rank(mut self, min: u32) -> Self {
        self.rank_min = Some(min);
        self
    }

    pub fn with_max_rank(mut self, max: u32) -> Self {
        self.rank_max = Some(max);
        self
    }

    pub fn with_exact_rank(mut self, rank: u32) -> Self {
        self.rank_min = Some(rank);
        self.rank_max = Some(rank);
        self
    }
}
