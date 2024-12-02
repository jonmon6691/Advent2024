use itertools::Itertools;
use std::path::Path;

fn check_reading(reading: &[i32]) -> bool {
    // Subtract the reading from itself but shifted by one 'level', this creates a vector of increments/decraments
    // TODO: If the check could be done inside the iterator then an early return would be possible, right now this will always iterate to the end of the list
    let deltas: Vec<i32> = reading
        .iter()
        .tuple_windows()
        .map(|(a, b)| *b - *a)
        .collect();
    // Calculate min/max of the deltas and check safety conditions
    match crate::VecStats::from_vec(&deltas) {
        None => false, // Empty list case
        Some(d) => (d.min >= 1 && d.max <= 3) || (d.max <= -1 && d.min >= -3),
    }
}

pub fn do_d02_1() -> Result<usize, String> {
    Ok(
        crate::load_input_vec_of_vecs(Path::new("input/input02.txt"))?
            .iter()
            .filter(|reading| check_reading(reading))
            .count(),
    )
}

fn drop_i(data: &[i32], i: usize) -> Vec<i32> {
    data.iter()
        .enumerate()
        .filter_map(|(j, v)| if i != j { Some(*v) } else { None })
        .collect::<Vec<i32>>()
}

pub fn do_d02_2() -> Result<usize, String> {
    Ok(
        crate::load_input_vec_of_vecs(Path::new("input/input02.txt"))?
            .iter()
            .filter(|reading| {
                check_reading(reading)
                    || (0..reading.len()) // Try removing one level at a time until the reading passes (Probably not optimal)
                        .map(|i| check_reading(&drop_i(reading, i)))
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
