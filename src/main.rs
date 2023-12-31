#[derive(Clone, Copy)]
enum Marking {
    Red,
    Green,
    Blue,
    Yellow,
    Orange,
    Purple
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
            _ => Err(())
        }
    }
}

#[derive(Clone, Copy)]
struct Square {
    position: usize,
    marking: Marking
}

impl Square {
    fn new(value: (usize, Marking)) -> Self {
        let (position, marking) = value;
        Self { position, marking }
    }
}

#[derive(Clone)]
struct Board {
    squares: Vec<Square>
}

fn main() {
    let sqaures: Vec<Square> = include_str!("board.txt")
        .split(",")
        .enumerate()
        .map(|(idx, pattern)| Square::new((idx, Marking::try_from(pattern).unwrap())))
        .collect();
    let board = Board {
        squares: sqaures
    };
}
