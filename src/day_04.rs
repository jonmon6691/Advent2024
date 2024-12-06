use regex::Regex;
use std::{iter::once, path::Path};

pub fn do_d04_1() -> Result<usize, String> {
    let og = &crate::load_input_utf8(Path::new("input/input_04.txt"))?;
    let spec = Regex::new(r"XMAS").expect("Invalid regex");
    let ceps = Regex::new(r"SAMX").expect("Invalid regex");
    let line_len = og.lines().next().unwrap().len();

    let mut matches = 0;
    // Count XMAS's
    matches += spec.captures_iter(og).count();
    matches += ceps.captures_iter(og).count();

    // Rotate 90 degrees
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
    matches += ceps.captures_iter(&rotated).count();

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
    matches += ceps.captures_iter(&diag).count();

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
    matches += ceps.captures_iter(&diag).count();

    Ok(matches)
}

#[test]
fn test_day_04_1() {
    let obfuscated_answer = do_d04_1().map(|answer| dbg!(answer) ^ 0x55555555);
    assert_eq!(obfuscated_answer, Ok(1431658354));
}

pub fn do_d04_2() -> Result<usize, String> {
    let og = &crate::load_input_utf8(Path::new("input/input_04.txt"))?;
    let line_len = og.lines().next().unwrap().len() - 1;

    let down =
        Regex::new(&format!("^(?s)M.M.{{{line_len}}}A.{{{line_len}}}S.S")).expect("Invalid regex");
    let up =
        Regex::new(&format!("^(?s)S.S.{{{line_len}}}A.{{{line_len}}}M.M")).expect("Invalid regex");
    let right =
        Regex::new(&format!("^(?s)M.S.{{{line_len}}}A.{{{line_len}}}M.S")).expect("Invalid regex");
    let left =
        Regex::new(&format!("^(?s)S.M.{{{line_len}}}A.{{{line_len}}}S.M")).expect("Invalid regex");

    let matches = (0..(og.len() - line_len * 2 + 7))
        .map(|i| {
            down.find_iter(&og[i..]).count()
                + up.find_iter(&og[i..]).count()
                + right.find_iter(&og[i..]).count()
                + left.find_iter(&og[i..]).count()
        })
        .sum();

    Ok(matches)
}

#[test]
fn test_day_04_2() {
    let obfuscated_answer = do_d04_2().map(|answer| dbg!(answer) ^ 0x55555555);
    assert_eq!(obfuscated_answer, Ok(1431655113));
}
