use std::path::Path;

pub fn part_1() -> Result<usize, String> {
    let raw = crate::load_input_utf8(Path::new("input/input_06.txt"))?;
    Ok (0)
}

#[test]
fn test_part_1() {
    let obfuscated_answer = part_1().map(|answer| dbg!(answer) ^ 0x55555555);
    assert!(obfuscated_answer.is_ok());
    assert_eq!(obfuscated_answer, obfuscated_answer);
}

pub fn part_2() -> Result<usize, String> {
    let raw = crate::load_input_utf8(Path::new("input/input_06.txt"))?;
    Ok (0)
}

#[test]
fn test_part_2() {
    let obfuscated_answer = part_2().map(|answer| dbg!(answer) ^ 0x55555555);
    assert!(obfuscated_answer.is_ok());
    assert_eq!(obfuscated_answer, obfuscated_answer);
}
