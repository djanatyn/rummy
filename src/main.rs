use rand::prelude::SliceRandom;
use strum::{EnumIter, IntoEnumIterator};

#[derive(EnumIter, Debug, Clone, PartialEq)]
enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

#[derive(EnumIter, Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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
                    .is_some_and(|next_rank| rank.continuations().contains(&next_rank));
                matches_suit && matches_rank
            }
            Card::Joker => panic!("jokers unsupported"),
        }
    }
}

#[derive(Debug)]
enum Sequence {
    Complete(Vec<Card>),
    Partial(Vec<Card>),
    Invalid(Vec<Card>),
}

#[derive(Debug, Clone, PartialEq)]
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

/// ([partial? sequence], [rest of cards])
type SequenceIteration = (Vec<Card>, Vec<Card>);

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
            // all card match
            [true] => Sequence::Complete(cards.to_vec()),
            // no cards match
            [false] => Sequence::Invalid(cards.to_vec()),
            // some cards match
            _ => Sequence::Partial(cards.to_vec()),
        }
    }

    fn find(cards: &[Card]) -> Vec<FoundSequence> {
        let mut found: Vec<FoundSequence> = vec![];

        for (idx, card) in cards.iter().enumerate() {
            let rest: Vec<Card> = cards
                .iter()
                .enumerate()
                .filter_map(|(i, c)| (idx != i).then(|| c.clone()))
                .collect();
            let mut next: Vec<SequenceIteration> = vec![(vec![card.clone()], rest)];

            while let Some((sequence, rest)) = next.pop() {
                match &sequence[..] {
                    [one] => {
                        todo!()
                    }
                    [begin, end] | [begin, .., end] => {
                        todo!()
                    }
                    _ => {
                        todo!()
                    }
                }
            }

            // TODO: you only need to move forward because you'll always start with the smallest card
            // - you can remove the prepend work and only look at append work, you only have to extend ending continuations
            // TODO: avoid replicated work by checking equivalent subsequences?
            //
            // starting state: next = [item=[card] rest=[rest of the cards]]
            //
            // example:
            // next = [
            //    item=[2H] rest=[1H 3H 4H]
            // ]
            // completed = []
            //
            // while loop on next.pop(): ...
            //
            // loop(item=[2H] rest=[1H 3H 4H]):
            //   starting + ending continuations (singleton, append):
            //     2H.sequence(1H) -> []
            //     2H.sequence(3H) -> item=[2H, 3H] rest=[1H, 4H]
            //     2H.sequence(4H) -> []
            //   next = [
            //     item=[2H, 3H] rest=[1H, 4H]
            //   ]
            //   completed = []
            //
            //  loop(item=[2H, 3H] rest=[1H, 4H]):
            //    starting continuations (prepend):
            //      1H.sequence(2H) -> [1H, 2H] -> item=[1H, 2H, 3H] rest=[4H]
            //      4H.sequence(2H) -> []
            //    ending continuations (append):
            //      3H.sequence(1H) -> []
            //      3H.sequence(4H) -> [3H, 4H] -> item=[2H, 3H, 4H] rest=[1H]
            //    next = [
            //      item=[2H, 3H, 4H] rest=[1H]
            //      item=[1H, 2H, 3H] rest=[4H]
            //    ]
            //    completed = [
            //      item=[2H, 3H, 4H] rest=[1H]
            //      item=[1H, 2H, 3H] rest=[4H]
            //    ]
            //
            // loop(item=[2H, 3H, 4H] rest=[1H]):
            //   starting continuations (prepend):
            //     1H.sequence(2H) -> [1H, 2H, 3H, 4H] -> item=[1H, 2H, 3H, 4H] rest=[4H]
            //   ending continuations:
            //     4H.sequence(1H) -> []
            //   next = [
            //     item=[1H, 2H, 3H] rest=[4H]
            //   ]
            //   completed = [
            //      item=[2H, 3H, 4H] rest=[1H]
            //      item=[1H, 2H, 3H] rest=[4H]
            //      item=[1H, 2H, 3H, 4H] rest=[]
            //   ]
        }
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complete_sequences() {
        let heart_sequence_hand = vec![
            Card::Card((Suit::Hearts, Rank::Three)),
            Card::Card((Suit::Hearts, Rank::Four)),
            Card::Card((Suit::Hearts, Rank::Five)),
        ];

        assert!(matches!(
            Sequence::check(&heart_sequence_hand),
            Sequence::Complete(_)
        ));
    }

    #[test]
    fn test_partial_sequences() {
        let two_consecutive = vec![
            Card::Card((Suit::Hearts, Rank::Two)),
            Card::Card((Suit::Hearts, Rank::Four)),
            Card::Card((Suit::Hearts, Rank::Five)),
        ];

        assert!(matches!(
            Sequence::check(&two_consecutive),
            Sequence::Partial(_)
        ));
    }

    #[test]
    fn test_invalid_sequences() {
        let missing_continuation = vec![
            Card::Card((Suit::Hearts, Rank::Three)),
            Card::Card((Suit::Hearts, Rank::Five)),
            Card::Card((Suit::Hearts, Rank::Jack)),
        ];

        assert!(matches!(
            Sequence::check(&missing_continuation),
            Sequence::Invalid(_)
        ));

        let mismatched_suits = vec![
            Card::Card((Suit::Hearts, Rank::Three)),
            Card::Card((Suit::Spades, Rank::Four)),
            Card::Card((Suit::Hearts, Rank::Five)),
        ];

        assert!(matches!(
            Sequence::check(&mismatched_suits),
            Sequence::Invalid(_)
        ));
    }
}
