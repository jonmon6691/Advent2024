use std::{
    collections::{HashMap, HashSet},
    iter::once,
    ops::Range,
    path::Path,
};

use rayon::prelude::*;

pub fn do_d06_1() -> Result<usize, String> {
    let raw = crate::load_input_utf8(Path::new("input/input_06.txt"))?;
    let grid = Grid::from_string(&raw)?;
    Ok(HashSet::<Position>::from_iter(once(grid.guard).chain(grid.map(|step| step.guard))).len())
}

#[test]
fn test_day_06_1() {
    let obfuscated_answer = do_d06_1().map(|answer| dbg!(answer) ^ 0x55555555);
    assert_eq!(obfuscated_answer, Ok(1431650694));
}

pub fn do_d06_2() -> Result<usize, String> {
    let raw = crate::load_input_utf8(Path::new("input/input_06.txt"))?;
    let grid = Grid::from_string(&raw)?;

    // Set of all positions the guard will occupy, essentially the result of part 1
    let trials: Vec<Position> = HashSet::<Position>::from_iter(grid.clone().map(|step| step.guard))
        .iter()
        .copied()
        .collect();

    Ok(trials
        .par_iter() // Parallelize the  search, one thread per trial
        .filter(|new_obj| !grid.clone().obstacles.contains(new_obj))
        .filter(|new_obj| grid.with_new_obstacle(**new_obj).would_loop())
        .count())
}

#[test]
fn test_day_06_2() {
    let obfuscated_answer = do_d06_2().map(|answer| dbg!(answer) ^ 0x55555555);
    assert_eq!(obfuscated_answer, Ok(1431654977));
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn moved(&self, dir: Direction) -> Option<Position> {
        Some(match dir {
            Direction::Up => Position {
                x: self.x.checked_sub(1)?,
                y: self.y,
            },
            Direction::Down => Position {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Left => Position {
                x: self.x,
                y: self.y.checked_sub(1)?,
            },
            Direction::Right => Position {
                x: self.x,
                y: self.y + 1,
            },
        })
    }
}

#[derive(Clone, Debug)]
pub struct Grid {
    guard: Position,
    heading: Direction,
    obstacles: HashSet<Position>,
    collisions: HashSet<(Position, Direction)>,
    x_max: Range<usize>,
    y_max: Range<usize>,
}

impl Grid {
    pub fn from_string(raw: &str) -> Result<Grid, String> {
        let mut map: HashMap<char, HashSet<Position>> = HashMap::new();
        for (x, line) in raw.lines().enumerate() {
            for (y, c) in line.chars().enumerate() {
                map.entry(c).or_default().insert(Position { x, y });
            }
        }

        match map.get(&'^').map(HashSet::len) {
            None | Some(0) => return Err("Error: No guard!".to_owned()),
            Some(2..) => return Err("Error: More than one guard, You're cooked!".to_owned()),
            Some(1) => {}
        }

        Ok(Grid {
            guard: *map[&'^'].iter().next().unwrap(),
            heading: Direction::Up,
            obstacles: map[&'#'].clone(),
            collisions: HashSet::new(),
            x_max: 0..raw.lines().count(),
            y_max: 0..raw.lines().last().unwrap().chars().count(),
        })
    }

    fn with_new_obstacle(&self, new_obj: Position) -> Self {
        let mut new = self.clone();
        new.obstacles.insert(new_obj);
        new
    }

    fn next_index(&self) -> Option<Position> {
        let i = self.guard.moved(self.heading)?;
        (self.x_max.contains(&i.x) && self.y_max.contains(&i.y)).then_some(i)
    }

    pub fn would_loop(&mut self) -> bool {
        if let Some(final_state) = self.last() {
            final_state.next_index().is_some()
        } else {
            false
        }
    }
}

impl Iterator for Grid {
    type Item = Grid;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.next_index()?;
        if self.obstacles.contains(&i) {
            let collision = (i, self.heading);
            if !self.collisions.insert(collision) {
                // Loop detected!
                return None;
            };
            self.heading = self.heading.turn();
        } else {
            self.guard = i;
        };
        Some(self.clone())
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn turn(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}
