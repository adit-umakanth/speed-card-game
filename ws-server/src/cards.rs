use Suit::*;

use rand::{seq::SliceRandom, thread_rng};

#[derive(Clone, Copy, Debug)]
enum Suit {
    Diamond,
    Heart,
    Spade,
    Club,
}

#[derive(Debug)]
struct Card {
    suit: Suit,
    rank: u8,
}

type TwoCardPiles = [Vec<Card>; 2];
type PlayerHand = [Card; 4];

struct SpeedTable {
    player_piles: TwoCardPiles,
    active_piles: TwoCardPiles,
    middle_piles: TwoCardPiles,
    player_hands: TwoCardPiles,
}

struct PlayerView {
    player_hand: Vec<Card>,
    active_cards: [Card; 2],
}

fn create_piles(deck: &mut Vec<Card>, i: usize) -> TwoCardPiles {
    [deck.drain(0..i).collect(), deck.drain(0..i).collect()]
}

impl SpeedTable {
    fn new() -> SpeedTable {
        let mut deck: Vec<Card> = Vec::new();
        let suits = [Diamond, Heart, Spade, Club];
        for suit in suits {
            for rank in 1..14 {
                deck.push(Card { suit, rank });
            }
        }

        deck.shuffle(&mut thread_rng());

        let middle_piles: TwoCardPiles = create_piles(&mut deck, 7);
        let player_piles: TwoCardPiles = create_piles(&mut deck, 19);
        let active_piles = TwoCardPiles::default();
        let player_hands = TwoCardPiles::default();

        SpeedTable {
            player_piles,
            active_piles,
            middle_piles,
            player_hands,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SpeedTable;

    #[test]
    fn create_table_pile_sizes() {
        let speed_table = SpeedTable::new();

        assert_eq!(speed_table.middle_piles[0].len(), 7);
        assert_eq!(speed_table.middle_piles[1].len(), 7);

        assert_eq!(speed_table.player_piles[0].len(), 19);
        assert_eq!(speed_table.player_piles[1].len(), 19);
    }
}
