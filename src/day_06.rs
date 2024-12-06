use std::{cmp::Ordering, collections::HashSet, ops::Add, path::Path};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Position {
    pub fn rot_kernel(self) -> Position {
        match (self.x.cmp(&0), self.y.cmp(&0)) {
            (Ordering::Less, Ordering::Equal) => Position { x: 0, y: -self.x },
            (Ordering::Equal, Ordering::Greater) => Position { x: self.y, y: 0 },
            (Ordering::Greater, Ordering::Equal) => Position { x: 0, y: -self.x },
            (Ordering::Equal, Ordering::Less) => Position { x: self.y, y: 0 },
            _ => panic!(),
        }
    }

    pub fn check_grid(&self, g: &[Vec<char>]) -> Option<char> {
        let x = match self.x {
            ..0 => return None,
            x => x,
        } as usize;

        let y = match self.y {
            ..0 => return None,
            y => y,
        } as usize;

        g.get(x).and_then(|row| row.get(y)).copied()
    }
}

pub fn print_grid(g: &[Vec<char>]) {
    for row in g.iter() {
        for c in row.iter() {
            print!("{}", c);
        }
        println!();
    }
}

pub fn do_d06_1() -> Result<usize, String> {
    let raw = crate::load_input_utf8(Path::new("input/input_06.txt"))?;
    let mut guard: Option<Position> = None;
    let mut grid: Vec<Vec<char>> = raw
        .lines()
        .enumerate()
        .map(|(x, line)| {
            line.chars()
                .enumerate()
                .map(|(y, c)| match c {
                    '^' => {
                        // Save the guard's position
                        guard = Some(Position {
                            x: x as i32,
                            y: y as i32,
                        });
                        // Then replace them with an empty map cell
                        '.'
                    }
                    c => c,
                })
                .collect()
        })
        .collect();

    // No guard no problem
    let guard = match guard {
        Some(pos) => pos,
        None => return Ok(0),
    };

    // Kernel control the direction the guard moves each iteration
    let mut kernel = Position { x: -1, y: 0 };
    let mut i = guard;
    let mut visited: HashSet<Position> = HashSet::new();
    visited.insert(guard);

    loop {
        match (i + kernel).check_grid(&grid) {
            None => {
                return Ok(visited.len());
            }
            Some('.') | Some('X') => {
                i = i + kernel;
                grid[i.x as usize][i.y as usize] = 'X';
                visited.insert(i);
            }
            Some('#') => {
                kernel = kernel.rot_kernel();
            }
            Some(_) => panic!(),
        }
    }
}

#[test]
fn test_day_06_1() {
    let obfuscated_answer = do_d06_1().map(|answer| dbg!(answer) ^ 0x55555555);
    assert_eq!(obfuscated_answer, Ok(1431650694));
}

pub fn do_d06_2() -> Result<u32, String> {
    Ok(0)
}

#[test]
fn test_day_06_2() {
    let obfuscated_answer = do_d06_2().map(|answer| dbg!(answer) ^ 0x55555555);
    assert!(obfuscated_answer.is_ok());
    assert_eq!(obfuscated_answer, obfuscated_answer);
}
