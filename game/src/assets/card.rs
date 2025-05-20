use bevy::asset::AssetPath;
use enum_iterator::all;

use crate::card::{Card, CardSuit};

impl From<&Card> for AssetPath<'_> {
    fn from(card: &Card) -> Self {
        let s = match card.suit() {
            CardSuit::Hearts => "H",
            CardSuit::Diamonds => "D",
            CardSuit::Clubs => "C",
            CardSuit::Spades => "S",
        };

        let r = match card.rank() {
            1 => "A".to_string(),
            11 => "J".to_string(),
            12 => "Q".to_string(),
            13 => "K".to_string(),
            2..=10 => card.rank().to_string(),
            _ => unreachable!(),
        };

        AssetPath::from(format!("cards/{}{}.png", s, r))
    }
}

pub(super) fn all_cards() -> impl Iterator<Item = Card> {
    all::<CardSuit>()
        .flat_map(move |suit| (1..=13).map(move |rank| Card::new_unchecked(suit, rank)))
}
