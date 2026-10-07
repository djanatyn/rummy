use rand::prelude::SliceRandom;
use strum::{EnumIter, IntoEnumIterator};

#[derive(EnumIter, Debug, Clone)]
enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

#[derive(EnumIter, Debug, Clone)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Clone)]
enum Card {
    Card((Suit, Rank)),
    Joker,
}

#[derive(Debug)]
struct Deck(Vec<Card>);

impl Rank {
    fn continuations(&self) -> Vec<Rank> {
        match self {
            Rank::Two => vec![Rank::Ace, Rank::Three],
            Rank::Three => vec![Rank::Two, Rank::Four],
            Rank::Four => vec![Rank::Three, Rank::Five],
            Rank::Five => vec![Rank::Four, Rank::Six],
            Rank::Six => vec![Rank::Five, Rank::Seven],
            Rank::Seven => vec![Rank::Six, Rank::Eight],
            Rank::Eight => vec![Rank::Seven, Rank::Nine],
            Rank::Nine => vec![Rank::Eight, Rank::Ten],
            Rank::Ten => vec![Rank::Nine, Rank::Jack],
            Rank::Jack => vec![Rank::Ten, Rank::Queen],
            Rank::Queen => vec![Rank::Jack, Rank::King],
            Rank::King => vec![Rank::Queen, Rank::Ace],
            Rank::Ace => vec![Rank::King, Rank::Two],
        }
    }
}

impl Deck {
    fn new() -> Self {
        let mut deck: Vec<Card> = Suit::iter()
            .flat_map(|suit| Rank::iter().map(move |rank| Card::Card((suit.clone(), rank))))
            .collect();
        deck.append(&mut vec![Card::Joker, Card::Joker]);

        Deck(deck)
    }

    fn shuffle(&mut self) {
        self.0.shuffle(&mut rand::rng());
    }

    fn draw(&mut self, cards: usize) -> Vec<Card> {
        let tmp = self.0.clone();
        let (drawn, rest) = tmp.split_at(cards);
        self.0 = rest.to_vec();
        drawn.to_vec()
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    let drawn = &deck.draw(13);

    println!("cards drawn: {:?}", &drawn);
    println!("===");
    println!("remaining deck: {:?}", &deck);
}

impl Card {
    fn sequence(&self, next: &Card) -> bool {
        todo!()
    }
}

#[derive(Debug)]
enum Sequence {
    Complete(Vec<Card>),
    Partial(Vec<Card>),
    Invalid(Vec<Card>),
}

impl Sequence {
    fn check(cards: &[Card]) -> Self {
        // TODO: jokers can only be used in a sequence once
        // for now, skip jokers
        let mut matches: Vec<bool> = cards
            .iter()
            .enumerate()
            .map(|(idx, card)| {
                // do any other cards match the select card?
                cards
                    .iter()
                    .enumerate()
                    .any(|(comparison_idx, comparison_card)| {
                        // don't compare card to itself
                        (idx != comparison_idx) && comparison_card.sequence(card)
                    })
            })
            .collect();
        matches.dedup();
        match matches[..] {
            [true] => Sequence::Complete(cards.to_vec()),
            [false] => Sequence::Invalid(cards.to_vec()),
            _ => Sequence::Partial(cards.to_vec()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn find_continuations() -> Result<(), String> {
        let hand = vec![
            Card::Card((Suit::Hearts, Rank::Two)),
            Card::Card((Suit::Hearts, Rank::Four)),
            Card::Card((Suit::Hearts, Rank::Five)),
        ];
    }
}
