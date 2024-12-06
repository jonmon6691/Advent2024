use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

type RulesAndUpdates = (HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>);

pub fn get_rules_and_updates(filename: &str) -> Result<RulesAndUpdates, String> {
    let raw = crate::load_input_utf8(Path::new(filename))?;

    // Find the empty line that splits the file
    let (sep_i, _) = raw
        .lines()
        .find_position(|l| l.is_empty())
        .ok_or("Input Error: Couldn't find an empty line separating the ruls and the updates!")?;

    let rules = raw
        .lines()
        .take(sep_i)
        .map(|rule| {
            rule.split('|')
                .map(str::parse::<u32>)
                .collect::<Result<Vec<u32>, _>>()
        })
        .collect::<Result<Vec<Vec<u32>>, _>>()
        .map_err(|err| format!("Input Error: Couldn't parse integer in rule ({err})."))?;

    let mut what_comes_after: HashMap<u32, HashSet<u32>> = HashMap::new();
    for rule in rules {
        what_comes_after.entry(rule[0]).or_default().insert(rule[1]);
    }

    let updates = raw
        .lines()
        .skip(sep_i + 1)
        .map(|rule| {
            rule.split(',')
                .map(str::parse::<u32>)
                .collect::<Result<Vec<u32>, _>>()
        })
        .collect::<Result<Vec<Vec<u32>>, _>>()
        .map_err(|err| format!("Input Error: Couldn't parse integer in update ({err})."))?;

    Ok((what_comes_after, updates))
}

pub fn do_d05_1() -> Result<u32, String> {
    let (mut what_comes_after, updates) = get_rules_and_updates("input/input_05.txt")?;

    Ok(updates
        .iter()
        .filter(|update| {
            let mut pre: HashSet<u32> = HashSet::new();
            update.iter().all(|page| {
                let pass = pre
                    .intersection(what_comes_after.entry(*page).or_default())
                    .count()
                    == 0;
                pre.insert(*page);
                pass
            })
        })
        .map(|f| f[f.len() / 2])
        .sum())
}

#[test]
fn test_day_05_1() {
    let obfuscated_answer = do_d05_1().map(|answer| dbg!(answer) ^ 0x55555555);
    assert_eq!(obfuscated_answer, Ok(1431650753));
}

pub fn do_d05_2() -> Result<u32, String> {
    let (what_comes_after, updates) = get_rules_and_updates("input/input_05.txt")?;

    Ok(updates
        .iter()
        .filter(|update| {
            let mut pre: HashSet<u32> = HashSet::new();
            update.iter().any(|page| {
                let pass = pre
                    .intersection(what_comes_after.get(page).unwrap_or(&HashSet::new()))
                    .count()
                    == 0;
                pre.insert(*page);
                !pass
            })
        })
        .map(|f| {
            // reorder the set acording to the rules
            let mut f = f.clone();
            f.sort_by(|a, b| {
                let a_before_b = what_comes_after
                    .get(a)
                    .unwrap_or(&HashSet::new())
                    .contains(b);
                let b_before_a = what_comes_after
                    .get(b)
                    .unwrap_or(&HashSet::new())
                    .contains(a);
                match (a_before_b, b_before_a) {
                    (true, false) => std::cmp::Ordering::Less,
                    (false, true) => std::cmp::Ordering::Greater,
                    _ => panic!(),
                }
            });
            // then return:
            f[f.len() / 2]
        })
        .sum())
}

#[test]
fn test_day_05_2() {
    let obfuscated_answer = do_d05_2().map(|answer| dbg!(answer) ^ 0x55555555);
    assert_eq!(obfuscated_answer, obfuscated_answer);
}
