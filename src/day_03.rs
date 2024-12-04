use regex::Regex;
use std::path::Path;

pub fn do_d03_1() -> Result<i32, String> {
    let spec = Regex::new(r"mul\((?<a>-?\d+),(?<b>-?\d+)\)").expect("Invalid regex");

    Ok(spec
        .captures_iter(&crate::load_input_utf8(Path::new("input/input_03.txt"))?)
        .map(|capture| capture["a"].parse::<i32>().unwrap() * capture["b"].parse::<i32>().unwrap())
        .sum())
}

pub fn do_d03_2() -> Result<i32, String> {
    let spec = Regex::new(r"(?<cmd>mul\((?<a>-?\d+),(?<b>-?\d+)\)|do\(\)|don't\(\))")
        .expect("Invalid regex");

    Ok(spec
        .captures_iter(&crate::load_input_utf8(Path::new("input/input_03.txt"))?)
        .fold((true, 0), |(do_mul, acc), capture| match &capture["cmd"] {
            s if s.starts_with("mul") => (
                do_mul,
                acc + if do_mul { 1 } else { 0 }
                    * capture["a"].parse::<i32>().unwrap()
                    * capture["b"].parse::<i32>().unwrap(),
            ),
            "don't()" => (false, acc),
            "do()" => (true, acc),
            _ => panic!(), // Can't get here with a proper regex
        })
        .1)
}

#[test]
fn test_day_03_1() {
    let obfuscated_answer = do_d03_1().map(|answer| dbg!(answer) ^ 0x55555555);
    assert_eq!(obfuscated_answer, Ok(1601593224));
}

#[test]
fn test_day_03_2() {
    let obfuscated_answer = do_d03_2().map(|answer| dbg!(answer) ^ 0x55555555);
    assert_eq!(obfuscated_answer, Ok(1374013564));
}
