use rand::prelude::SliceRandom;
use strum::{EnumIter, IntoEnumIterator};

#[derive(EnumIter, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

#[derive(EnumIter, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Card((Suit, Rank)),
    Joker,
}

#[derive(Debug)]
struct Deck(Vec<Card>);

impl Rank {
    fn continuations(&self) -> Rank {
        match self {
            Rank::Two => Rank::Three,
            Rank::Three => Rank::Four,
            Rank::Four => Rank::Five,
            Rank::Five => Rank::Six,
            Rank::Six => Rank::Seven,
            Rank::Seven => Rank::Eight,
            Rank::Eight => Rank::Nine,
            Rank::Nine => Rank::Ten,
            Rank::Ten => Rank::Jack,
            Rank::Jack => Rank::Queen,
            Rank::Queen => Rank::King,
            Rank::King => Rank::Ace,
            Rank::Ace => Rank::Two,
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
    let found = FoundSequence::find(&drawn);

    println!("cards drawn: {:?}", &drawn);
    println!("===");
    println!("remaining deck: {:?}", &deck);
    println!("===");
    println!("found sequences: {:?}", &found);
}

impl Card {
    fn rank(&self) -> Option<Rank> {
        match self {
            Card::Card((suit, rank)) => Some(rank.clone()),
            Card::Joker => None,
        }
    }

    fn suit(&self) -> Option<Suit> {
        match self {
            Card::Card((suit, rank)) => Some(suit.clone()),
            Card::Joker => None,
        }
    }

    fn sequence(&self, next: &Card) -> bool {
        match self {
            Card::Card((suit, rank)) => {
                let matches_suit: bool = next.suit().is_some_and(|next_suit| *suit == next_suit);
                let matches_rank: bool = next
                    .rank()
                    .is_some_and(|next_rank| rank.continuations() == next_rank);
                matches_suit && matches_rank
            }
            Card::Joker => true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum FoundSequence {
    Partial {
        sequence: Vec<Card>,
        rest: Vec<Card>,
    },
    Complete {
        sequence: Vec<Card>,
        rest: Vec<Card>,
    },
}

// index of used cards in a sequence, rest of cards are unselected
type SequenceIteration = Vec<usize>;

impl FoundSequence {
    fn find(cards: &[Card]) -> Vec<FoundSequence> {
        let mut found_sequences: Vec<FoundSequence> = vec![];
        let length = cards.len();

        for idx in 0..length {
            // pick a card and treat it as a potential sequence
            let mut iterations: Vec<SequenceIteration> = vec![vec![idx]];

            // find all possible sequences using remaining cards, recursively
            while let Some(used) = iterations.pop() {
                match &used[..] {
                    // found partial or complete sequence
                    [end] | [_, end] | [_, .., end] => {
                        // look at all remaining cards in hand, ignoring used cards
                        for next in (0..length).filter(|i| !(used.contains(i))) {
                            if cards[*end].sequence(&cards[next]) {
                                // found sequence indexes
                                let mut found = used.clone();
                                found.push(next);

                                // populate found sequence struct from indexes
                                let mut sequence = vec![];
                                let mut rest = vec![];

                                for (index, card) in cards.iter().enumerate() {
                                    if found.contains(&index) {
                                        sequence.push(card.clone());
                                    } else {
                                        rest.push(card.clone());
                                    }
                                }

                                sequence.sort();
                                rest.sort();

                                // check if sequence is partial or complete
                                let found_sequence = match sequence.len() {
                                    2 => FoundSequence::Partial { sequence, rest },
                                    _ => FoundSequence::Complete { sequence, rest },
                                };

                                found_sequences.push(found_sequence);

                                // add found sequence (indexes) as another iteration
                                iterations.push(found);
                            }
                        }
                    }
                    // invalid?
                    _ => {
                        unreachable!("invaild sequence iteration")
                    }
                }
            }
        }

        found_sequences.sort();
        found_sequences.dedup();
        return found_sequences;
    }
}

#[cfg(test)]
mod tests {
    use crate::{Card, Card::*, FoundSequence::*, Rank, Rank::*, Suit::*};

    #[test]
    fn test_find_sequence() {
        let heart_sequence_hand = vec![
            Card((Hearts, Three)),
            Card((Hearts, Four)),
            Card((Hearts, Five)),
        ];

        let found = FoundSequence::find(&heart_sequence_hand);
        let expected = vec![
            Partial {
                sequence: vec![Card((Hearts, Three)), Card((Hearts, Four))],
                rest: vec![Card((Hearts, Five))],
            },
            Partial {
                sequence: vec![Card((Hearts, Four)), Card((Hearts, Five))],
                rest: vec![Card((Hearts, Three))],
            },
            Complete {
                sequence: vec![
                    Card((Hearts, Three)),
                    Card((Hearts, Four)),
                    Card((Hearts, Five)),
                ],
                rest: vec![],
            },
        ];

        assert_eq!(found, expected);
    }
}
