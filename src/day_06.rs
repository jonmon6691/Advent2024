use std::{
    collections::{HashMap, HashSet},
    iter::once,
    ops::Range,
    path::Path,
};

use crate::Direction;
use crate::Position;

use rayon::prelude::*;

pub fn do_d06_1() -> Result<usize, String> {
    let raw = crate::load_input_utf8(Path::new("input/input_06.txt"))?;
    let grid = Grid::from_string(&raw)?;
    Ok(HashSet::<Position>::from_iter(once(grid.guard.pos).chain(grid.map(|step| step.pos))).len())
}

#[test]
fn test_day_06_1() {
    let obfuscated_answer = do_d06_1().map(|answer| dbg!(answer) ^ 0x55555555);
    assert_eq!(obfuscated_answer, Ok(1431650694));
}

pub fn do_d06_2() -> Result<usize, String> {
    let raw = crate::load_input_utf8(Path::new("input/input_06.txt"))?;
    let grid = Grid::from_string(&raw)?;

    Ok(
        // Set of all positions the guard will occupy, essentially the result of part 1
        HashSet::<Position>::from_iter(grid.clone().map(|step| step.pos))
            .par_iter() // For each position the guard was in
            .filter(|new_obj| grid.with_new_obstacle(**new_obj).would_loop()) // try adding an obstacle there and check if the guard is in a loop
            .count(),
    )
}

#[test]
fn test_day_06_2() {
    let obfuscated_answer = do_d06_2().map(|answer| dbg!(answer) ^ 0x55555555);
    assert_eq!(obfuscated_answer, Ok(1431654977));
}

#[derive(Debug, Clone, Copy)]
pub struct Guard {
    pos: Position,
    dir: Direction,
}

#[derive(Clone, Debug)]
pub struct Grid {
    guard: Guard,
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

        match map.get(&'^') {
            None => Err("Error: No guard!".to_owned()),
            Some(g) if g.len() > 1 => Err("Error: More than one guard, You're cooked!".to_owned()),
            Some(g) => Ok(Grid {
                guard: Guard {
                    pos: *g.iter().next().unwrap(),
                    dir: Direction::Up,
                },
                obstacles: map[&'#'].clone(),
                collisions: HashSet::new(),
                x_max: 0..raw.lines().count(),
                y_max: 0..raw.lines().last().unwrap().chars().count(),
            }),
        }
    }

    fn with_new_obstacle(&self, new_obj: Position) -> Self {
        let mut new = self.clone();
        new.obstacles.insert(new_obj);
        new
    }

    fn next_index(&self) -> Option<Position> {
        let i = self.guard.pos.moved(self.guard.dir)?;
        (self.x_max.contains(&i.x) && self.y_max.contains(&i.y)).then_some(i)
    }

    pub fn would_loop(&mut self) -> bool {
        if let Some(final_state) = self.last() {
            if let Some(next) = final_state.pos.moved(final_state.dir) {
                return self.obstacles.contains(&next);
            }
        }
        false
    }
}

impl Iterator for Grid {
    type Item = Guard;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.next_index()?;
        if self.obstacles.contains(&i) {
            if !self.collisions.insert((i, self.guard.dir)) {
                // Loop detected!
                return None;
            };
            self.guard.dir = self.guard.dir.turn();
        } else {
            self.guard.pos = i;
        };
        Some(self.guard)
    }
}
