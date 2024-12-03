use itertools::Itertools;
use std::{cmp::Ordering, path::Path};

/// Checks the safety of a given reading
fn check_reading_safety(reading: &[i32]) -> bool {
    let mut direction: Option<Ordering> = None;
    reading
        .iter()
        .tuple_windows()
        .map(|(a, b)| {
            direction = direction.or(Some(b.cmp(a)));
            match *b - *a {
                -3..=-1 => direction == Some(Ordering::Less),
                1..=3 => direction == Some(Ordering::Greater),
                _ => false,
            }
        })
        .all(bool::into)
}

pub fn do_d02_1() -> Result<usize, String> {
    Ok(
        crate::load_input_vec_of_vecs(Path::new("input/input02.txt"))?
            .iter()
            .filter(|reading| check_reading_safety(reading))
            .count(),
    )
}

pub fn do_d02_2() -> Result<usize, String> {
    Ok(
        crate::load_input_vec_of_vecs(Path::new("input/input02.txt"))?
            .iter()
            .filter(|reading| {
                check_reading_safety(reading)
                    || (0..reading.len()) // Try removing one level at a time until the reading passes (Probably not optimal)
                        .map(|i| check_reading_safety(&crate::drop_i(reading, i)))
                        .any(bool::into)
            })
            .count(),
    )
}

#[test]
fn test_day_02_1() {
    let ans = do_d02_1();
    let day01_01_obfuscated_answer = ans.map(|answer| dbg!(answer) ^ 0x55555555);
    assert_eq!(day01_01_obfuscated_answer, Ok(1431655823));
}

#[test]
fn test_day_02_2() {
    let ans = do_d02_2();
    let day01_02_obfuscated_answer = ans.map(|answer| dbg!(answer) ^ 0x55555555);
    assert_eq!(day01_02_obfuscated_answer, Ok(1431655543));
}
