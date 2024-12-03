use itertools::Itertools;
use std::{cmp::Ordering, path::Path};

/// Checks the safety of a given reading
fn check_reading_safety(reading: &[i32]) -> bool {
    // Keeps track of an increasing or decreasing trend, None until the first pair is observed
    let mut direction: Option<Ordering> = None;
    reading
        .iter()
        .tuple_windows()
        .map(|(a, b)| {
            direction = direction.or(Some(b.cmp(a))); // .or is only taken on the first loop and sets direction to Some()thing
            match *b - *a {
                -3..=-1 => direction == Some(Ordering::Less),
                1..=3 => direction == Some(Ordering::Greater),
                _ => false, // 0 or ABS() > 3
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
                check_reading_safety(reading) // Check the nominal case (no deletions)
                    || (0..reading.len()) // OR try removing one level at a time until the reading passes (Probably not optimal)
                        .map(|i| check_reading_safety(&crate::drop_i(reading, i)))
                        .any(bool::into)
            })
            .count(),
    )
}

#[test]
fn test_day_02_1() {
    let obfuscated_answer = do_d02_1().map(|answer| dbg!(answer) ^ 0x55555555);
    assert_eq!(obfuscated_answer, Ok(1431655823));
}

#[test]
fn test_day_02_2() {
    let obfuscated_answer = do_d02_2().map(|answer| dbg!(answer) ^ 0x55555555);
    assert_eq!(obfuscated_answer, Ok(1431655543));
}
