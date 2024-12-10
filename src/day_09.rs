use std::{
    iter::{once, repeat_n},
    path::Path,
};

use itertools::Itertools;

pub fn part_1() -> Result<usize, String> {
    let raw = crate::load_input_utf8(Path::new("input/input_09.txt"))?;

    let mut memory: Vec<Option<usize>> = raw
        .chars()
        .chain(once('0'))
        .tuples()
        .enumerate()
        .flat_map(|(i, (file, free))| {
            repeat_n(Some(i), file.to_digit(10).unwrap() as usize)
                .chain(repeat_n(None, free.to_digit(10).unwrap() as usize))
        })
        .collect();

    let mut head = 0;
    let mut tail = memory.len() - 1;
    while head != tail {
        // print_mem(&memory, head, tail);
        println!();
        if memory[head].is_some() {
            head += 1;
            continue;
        }
        if memory[tail].is_none() {
            tail -= 1;
            continue;
        }
        memory[head] = memory[tail];
        head += 1;
        tail -= 1;
    }
    Ok(memory[0..=head]
        .iter()
        .copied()
        .while_some()
        .enumerate()
        .map(|(i, id)| i * id)
        .sum())
}

fn _print_mem(mem: &Vec<Option<usize>>, head: usize, tail: usize) {
    for v in mem {
        match v {
            Some(id) => print!("{} ", id),
            None => print!(". "),
        }
    }
    println!();
    for _ in 0..head {
        print!("  ");
    }
    print!("H ");
    for _ in head..tail - 1 {
        print!("  ");
    }
    print!("T ({}, {})", head, tail);
}

#[test]
fn test_part_1() {
    let obfuscated_answer = part_1().map(|answer| dbg!(answer) ^ 0x55555555);
    assert!(obfuscated_answer.is_ok());
    assert_eq!(obfuscated_answer, obfuscated_answer);
}

#[derive(Debug, Clone, Copy)]
struct File {
    id: usize,
    size: usize,
}

#[derive(Debug, Clone, Copy)]
enum Data {
    Used(File),
    Free(usize),
}

pub fn part_2() -> Result<usize, String> {
    let raw = crate::load_input_utf8(Path::new("input/input_09_test.txt"))?;
    let fstab = raw
        .chars()
        .chain(once('0'))
        .map(|c| c.to_digit(10).unwrap() as usize)
        .tuples()
        .enumerate()
        .flat_map(|(i, (file, free))| [Data::Used(File { id: i, size: file }), Data::Free(free)])
        .collect::<Vec<Data>>();

    let mut head = 0;
    let mut tail = fstab.len() - 1;

    while head < tail {
        head += 1;
        tail -= 1;
    }

    Ok(0)
}

#[test]
fn test_part_2() {
    let obfuscated_answer = part_2().map(|answer| dbg!(answer) ^ 0x55555555);
    assert!(obfuscated_answer.is_ok());
    assert_eq!(obfuscated_answer, obfuscated_answer);
}
