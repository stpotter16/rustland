use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Clone, Copy)]
enum Marking {
    Red,
    Green,
    Blue,
    Yellow,
    Orange,
    Purple,
}

impl TryFrom<&str> for Marking {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "r" => Ok(Marking::Red),
            "g" => Ok(Marking::Green),
            "b" => Ok(Marking::Blue),
            "y" => Ok(Marking::Yellow),
            "o" => Ok(Marking::Orange),
            "p" => Ok(Marking::Purple),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Square {
    position: usize,
    marking: Marking,
}

impl Square {
    fn new(value: (usize, Marking)) -> Self {
        let (position, marking) = value;
        Self { position, marking }
    }
}

#[derive(Clone)]
struct Board {
    squares: Vec<Square>,
}

#[derive(Clone, Copy)]
struct Card {
    marking: Marking,
    double: bool,
}

struct Deck {
    cards: Vec<Card>,
}

impl TryFrom<&str> for Deck {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parsed_cards: Vec<Card> = Vec::new();
        let card_specs: Vec<&str> = value.trim().split(",").collect();

        for card_spec in card_specs.into_iter() {
            let (card_count, card_type) = card_spec.split_at(1);
            let to_add = card_count.parse::<usize>().unwrap();
            let card = match card_type {
                "r" => Card {
                    marking: Marking::Red,
                    double: false,
                },
                "g" => Card {
                    marking: Marking::Green,
                    double: false,
                },
                "b" => Card {
                    marking: Marking::Blue,
                    double: false,
                },
                "y" => Card {
                    marking: Marking::Yellow,
                    double: false,
                },
                "o" => Card {
                    marking: Marking::Orange,
                    double: false,
                },
                "p" => Card {
                    marking: Marking::Purple,
                    double: false,
                },
                _ => return Err(()),
            };

            for _ in 0..to_add {
                parsed_cards.push(card.clone())
            }
        }
        parsed_cards.shuffle(&mut thread_rng());

        Ok(Deck {
            cards: parsed_cards.clone(),
        })
    }
}

fn main() {
    // Parse the board
    let squares: Vec<Square> = include_str!("board.txt")
        .trim()
        .split(",")
        .enumerate()
        .map(|(idx, pattern)| Square::new((idx, Marking::try_from(pattern).unwrap())))
        .collect();
    let board = Board { squares: squares };

    for square in board.squares.iter() {
        dbg!(square);
    }
}
