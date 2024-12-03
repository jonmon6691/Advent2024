use itertools::Itertools;
use std::{cmp::Ordering, path::Path};

/// Checks the safety of a given reading
///
/// Note: `.tuple_windows()` won't emit an item if `reading` is 0 or 1 items
/// in length. Furthermore, .all() returns true for an empty iterator.
/// Therefore this function can be considered optimistic! It assumes that
/// no news is good news, and that a single "level" is mighty fine indeed.
fn check_safety(reading: &[i32]) -> bool {
    // Keeps track of an increasing or decreasing trend, None until the first pair is observed
    let mut trend: Option<Ordering> = None;
    reading
        .iter()
        .tuple_windows()
        .map(|(a, b)| {
            trend = trend.or(Some(b.cmp(a))); // .or is only taken on the first loop and sets trend to Some()thing
            match *b - *a {
                -3..=-1 => trend == Some(Ordering::Less),
                1..=3 => trend == Some(Ordering::Greater),
                _ => false, // 0 or ABS(diff) > 3
            }
        })
        .all(bool::into)
}

pub fn do_d02_1() -> Result<usize, String> {
    Ok(
        crate::load_input_vec_of_vecs(Path::new("input/input02.txt"))?
            .iter()
            .filter(|reading| check_safety(reading))
            .count(),
    )
}

pub fn do_d02_2() -> Result<usize, String> {
    Ok(
        crate::load_input_vec_of_vecs(Path::new("input/input02.txt"))?
            .iter()
            .filter(|reading| {
                check_safety(reading) // Check the nominal case (no deletions)
                    || (0..reading.len()) // OR try removing one level at a time until the reading passes (Probably not optimal)
                        .map(|i| check_safety(&crate::drop_i(reading, i)))
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
