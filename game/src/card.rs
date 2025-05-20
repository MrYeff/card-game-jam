use bevy::{ecs::component::Component, prelude::Deref};
use enum_iterator::Sequence;

#[derive(Clone, Copy, Sequence, PartialEq, Eq)]
#[repr(usize)]
pub enum CardSuit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Clone, Component)]
#[component(immutable)]
pub struct Card {
    suit: CardSuit,
    rank: u32,
}

#[derive(Debug)]
pub struct InvalideCardError;

impl Card {
    #[inline]
    pub const fn new(suit: CardSuit, rank: u32) -> Self {
        match Self::try_new(suit, rank) {
            Ok(t) => t,
            Err(e) => panic!(),
        }
    }

    #[inline]
    pub const fn try_new(suit: CardSuit, rank: u32) -> Result<Self, InvalideCardError> {
        if !matches!(rank, (1..=13)) {
            Err(InvalideCardError)
        } else {
            Ok(Self::new_unchecked(suit, rank))
        }
    }

    #[inline(always)]
    pub const fn new_unchecked(suit: CardSuit, rank: u32) -> Card {
        Card { suit, rank }
    }

    #[inline(always)]
    pub fn suit(&self) -> CardSuit {
        self.suit
    }

    #[inline(always)]
    pub fn rank(&self) -> u32 {
        self.rank
    }
}
