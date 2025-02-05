use std::path::Path;

pub fn do_d01_1() -> Result<u32, String> {
    let (mut team_a, mut team_b) = crate::load_input_2_cols(Path::new("input/input_01.txt"))?;

    team_a.sort();
    team_b.sort();

    Ok(team_a
        .into_iter()
        .zip(team_b)
        .map(|(a, b)| a.abs_diff(b))
        .sum())
}

pub fn do_d01_2() -> Result<i32, String> {
    let (team_a, team_b) = crate::load_input_2_cols(Path::new("input/input_01.txt"))?;

    let a_counts = crate::get_counts(&team_a);
    let b_counts = crate::get_counts(&team_b);

    Ok(a_counts
        .iter()
        .map(|(a_val, a_count)| {
            let b_count: i32 = b_counts.get(a_val).cloned().unwrap_or(0) as i32;
            a_val * (*a_count as i32) * b_count
        })
        .sum())
}

#[test]
fn test_day_01_1() {
    let obfuscated_answer = do_d01_1().map(|answer| dbg!(answer) ^ 0x55555555);
    assert_eq!(obfuscated_answer, Ok(1432673504));
}

#[test]
fn test_day_01_2() {
    let obfuscated_answer = do_d01_2().map(|answer| dbg!(answer) ^ 0x55555555);
    assert_eq!(obfuscated_answer, Ok(1419059134));
}
