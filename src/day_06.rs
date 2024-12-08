use std::{
    collections::{HashMap, HashSet},
    iter::once,
    ops::Range,
    path::Path,
};

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

    Ok(HashSet::<Position>::from_iter(
        grid.clone()
            .filter_map(|state| state.next_index())
            .filter(|new_obj| !grid.obstacles.contains(new_obj))
            .filter(|new_obj| {
                let mut au = grid.clone();
                au.obstacles.insert(*new_obj);
                au.would_loop()
            }),
    )
    .len())
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
            Direction::Up => Position { x: self.x.checked_sub(1)?, y: self.y },
            Direction::Down => Position { x: self.x + 1, y: self.y },
            Direction::Left => Position { x: self.x, y: self.y.checked_sub(1)? },
            Direction::Right => Position { x: self.x, y: self.y + 1 },
        })
    }
}

#[derive(Clone, Debug)]
pub struct Grid {
    guard: Position, // Current position
    heading: Direction,    // Kernel aka direction
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

    pub fn print(&self) {
        for x in self.x_max.clone() {
            for y in self.y_max.clone() {
                let i = Position { x, y };
                match (i == self.guard, self.obstacles.contains(&i)) {
                    (true, _) => print!("^"),
                    (_, true) => print!("#"),
                    _ => print!("."),
                }
            }
            println!();
        }
    }

    pub fn print_with_highlight(&self, hl: Position) {
        for x in self.x_max.clone() {
            for y in self.y_max.clone() {
                let i = Position { x, y };
                if i == hl {
                    print!("O");
                } else if i == self.guard {
                    print!("^")
                } else if self.obstacles.contains(&i) {
                    print!("#")
                } else {
                    print!(".")
                }
            }
            println!();
        }
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
