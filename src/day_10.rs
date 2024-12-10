use crate::Position;
use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

type Board = HashMap<u32, HashSet<Position>>;

fn load_board() -> Result<Board, String> {
    let raw = crate::load_input_utf8(Path::new("input/input_10.txt"))?;

    let mut board: Board = HashMap::new();
    for (x, line) in raw.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            let h = c.to_digit(10).unwrap();
            board.entry(h).or_default().insert(Position { x, y });
        }
    }
    Ok(board)
}

// Returns the number of 9's reachable from this spot
fn nines_from_here<'a>(h: u32, p: &'a Position, b: &'a Board) -> HashSet<&'a Position> {
    if h == 9 {
        // Base case
        return HashSet::from_iter([p]);
    }
    HashSet::from_iter(
        b[&(h + 1)]
            .iter()
            .filter(|neighbor| p.distance(neighbor) == 1)
            .flat_map(|neighbor| nines_from_here(h + 1, neighbor, b)),
    )
}

pub fn part_1() -> Result<usize, String> {
    let board = load_board()?;
    Ok(board[&0]
        .iter()
        .map(|zpos| nines_from_here(0, zpos, &board).len())
        .sum())
}

#[test]
fn test_part_1() {
    let obfuscated_answer = part_1().map(|answer| dbg!(answer) ^ 0x55555555);
    assert_eq!(obfuscated_answer, Ok(1431656271));
}

// Returns the number of ways to reach a 9 from this spot
fn ways_to_nine<'a>(h: u32, p: &'a Position, b: &'a Board) -> usize {
    if h == 9 {
        return 1;
    }
    b[&(h + 1)]
        .iter()
        .filter(|neighbor| p.distance(neighbor) == 1)
        .map(|neighbor| ways_to_nine(h + 1, neighbor, b))
        .sum()
}

pub fn part_2() -> Result<usize, String> {
    let board = load_board()?;
    Ok(board[&0]
        .iter()
        .map(|zpos| ways_to_nine(0, zpos, &board))
        .sum())
}

#[test]
fn test_part_2() {
    let obfuscated_answer = part_2().map(|answer| dbg!(answer) ^ 0x55555555);
    assert_eq!(obfuscated_answer, Ok(1431654659));
}
