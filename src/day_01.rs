use std::path::Path;

pub fn do_d01_1() -> i32 {
    let input_path = Path::new("input/input01-1.txt");
    let (mut team_a, mut team_b) =
        crate::load_input_2_cols(&input_path).expect("Error loading input");
    team_a.sort();
    team_b.sort();
    team_a
        .into_iter()
        .zip(team_b.into_iter())
        .map(|(a, b)| -> i32 { (a - b).abs() })
        .sum()
}

pub fn do_d01_2() -> i32 {
    let input_path = Path::new("input/input01-1.txt");
    let (team_a, team_b) =
        crate::load_input_2_cols(&input_path).expect("Error loading input");
    let a_counts = crate::get_counts(&team_a);
    let b_counts = crate::get_counts(&team_b);
    a_counts.iter().map(|(a_val, a_count)| {
        let b_count: i32 = b_counts.get(a_val).cloned().unwrap_or(0) as i32;
        a_val * (*a_count as i32) * b_count
    }).sum()
}

#[test]
fn test_day_01_1() {
    let ans = do_d01_1();
    let day01_01_obfuscated_answer = 0x55555555 ^ ans;
    assert_eq!(dbg!(day01_01_obfuscated_answer), 1432673504);
}

#[test]
fn test_day_01_2() {
    let ans = do_d01_2();
    let day01_02_obfuscated_answer = 0x55555555 ^ ans;
    assert_eq!(dbg!(day01_02_obfuscated_answer), 1419059134);
}
