
fn main() {
    let board = include_str!("board.txt");
    for line in board.split(",") {
       println!("{line}");
    }
}
