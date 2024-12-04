use regex::Regex;
use std::{iter::once, path::Path};

pub fn do_d04_1() -> Result<usize, String> {
    let og = &crate::load_input_utf8(Path::new("input/input_04.txt"))?;
    let spec = Regex::new(r"XMAS").expect("Invalid regex");
    let line_len = og.lines().next().unwrap().len();

    // Count XMAS's
    let mut matches = spec.captures_iter(og).count();

    // Mirror left-right
    let flipped: String = og
        .lines()
        .map(|l| l.chars().rev().chain(once('\n')).collect::<String>())
        .collect();
    matches += spec.captures_iter(&flipped).count();

    // rotate
    let rotated: String = (0..line_len)
        .map(|i| {
            og.chars()
                .skip(i)
                .step_by(line_len + 1)
                .chain(once('\n'))
                .collect::<String>()
        })
        .collect();
    matches += spec.captures_iter(&rotated).count();

    // Mirror left/right again
    let flipped_and_rotated: String = rotated
        .lines()
        .map(|l| l.chars().rev().chain(once('\n')).collect::<String>())
        .collect();
    matches += spec.captures_iter(&flipped_and_rotated).count();

    // Diagonalize
    let square: Vec<Vec<char>> = og.lines().map(|l| l.chars().collect()).collect();
    let diag: String = (0..(2 * line_len - 1))
        .map(|i| {
            (0..line_len - (i).abs_diff(line_len - 1))
                .map(|j| {
                    let x = i.min(line_len - 1) - j;
                    let y = j + (i as i32 - (line_len as i32 - 1)).max(0) as usize;
                    square[x][y]
                })
                .chain(once('\n'))
                .collect::<String>()
        })
        .collect();
    matches += spec.captures_iter(&diag).count();

    // Mirror the diagonal
    let diag: String = diag
        .lines()
        .map(|l| l.chars().rev().chain(once('\n')).collect::<String>())
        .collect();
    matches += spec.captures_iter(&diag).count();

    // Diagonalize the other way by mirroring first
    let square: Vec<Vec<char>> = og.lines().map(|l| l.chars().rev().collect()).collect();
    let diag: String = (0..(2 * line_len - 1))
        .map(|i| {
            (0..line_len - (i).abs_diff(line_len - 1))
                .map(|j| {
                    let x = i.min(line_len - 1) - j;
                    let y = j + (i as i32 - (line_len as i32 - 1)).max(0) as usize;
                    square[x][y]
                })
                .chain(once('\n'))
                .collect::<String>()
        })
        .collect();
    matches += spec.captures_iter(&diag).count();

    // Mirror the diagonal
    let diag: String = diag
        .lines()
        .map(|l| l.chars().rev().chain(once('\n')).collect::<String>())
        .collect();
    matches += spec.captures_iter(&diag).count();

    Ok(matches)
}

#[test]
fn test_day_04_1() {
    let obfuscated_answer = do_d04_1().map(|answer| dbg!(answer) ^ 0x55555555);
    assert_eq!(obfuscated_answer, Ok(1431658354));
}

pub fn do_d04_2() -> Result<i32, String> {
    Err("TODO!".to_owned())
}

#[test]
fn test_day_04_2() {
    let obfuscated_answer = do_d04_2().map(|answer| dbg!(answer) ^ 0x55555555);
    assert_eq!(obfuscated_answer, obfuscated_answer);
}
