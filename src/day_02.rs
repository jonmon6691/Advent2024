use itertools::Itertools;
use std::{ops::Not, path::Path};

pub fn do_d02_1() -> Result<usize, String> {
    let readings = crate::load_input_vec_of_vecs(Path::new("input/input02.txt"))?;
    Ok(readings
        .iter()
        .map(|reading| check_reading(reading))
        .filter(|result| *result)
        .count())
}

fn test_delta(is_incr: bool, delta: i32) -> bool {
    (is_incr && (1..=3).contains(&delta)) || (is_incr.not() && (-3..=-1).contains(&delta))
}

fn check_reading(reading: &[i32]) -> bool {
    // Subtract the reading from itself but shifted by one 'level', this creates a vector of increments/decraments
    let deltas: Vec<i32> = reading
        .iter()
        .tuple_windows()
        .map(|(a, b)| *b - *a)
        .collect();
    // Calculate min/max of the deltas
    match crate::VecStats::from_vec(&deltas) {
        None => false, // Empty list case
        Some(d) => (d.min >= 1 && d.max <= 3) || (d.max <= -1 && d.min >= -3),
    }
}

pub fn do_d02_2() -> Result<usize, String> {
    let readings = crate::load_input_vec_of_vecs(Path::new("input/input02.txt"))?;

    Ok(readings
        .iter()
        .map(|r| {
            // Subtract the reading from itself but shifted by one 'level', this creates a vector of increments/decraments
            let deltas: Vec<i32> = r.iter().tuple_windows().map(|(a, b)| *b - *a).collect();
            let is_incr = deltas[0] > 0; // TODO: Fix bug if ==0, we don't actually know if its going up or down yet, also Handle empty list

            // Contains a list of indexes where a violation occured
            let violations: Vec<usize> = deltas
                .iter()
                .enumerate()
                .filter_map(|(i, d)| {
                    if test_delta(is_incr, *d) {
                        None
                    } else {
                        Some(i)
                    }
                })
                .collect();

            match violations.len() {
                0 => true, // No repairs needed
                1.. => {
                    (0..r.len()) // Try removing one level at a time until the reading passes (Probably not optimal)
                        .map(|i| {
                            check_reading(
                                &r.iter()
                                    .enumerate()
                                    .filter_map(|(j, v)| if i != j { Some(*v) } else { None })
                                    .collect::<Vec<i32>>(),
                            )
                        })
                        .any(|result| result)
                }
            }
        })
        .filter(|result| *result)
        .count())
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
