use std::collections::{HashMap, HashSet};

pub fn part1(inp: String) {
    let nums = inp
        .lines()
        .map(|c| c.parse().unwrap())
        .collect::<Vec<usize>>();
    let mut seen = HashSet::new();
    for x in nums.iter() {
        let key = 2020 - x;
        if seen.contains(&key) {
            println!("{}", key * x);
            return;
        } else {
            seen.insert(x);
        }
    }
}

pub fn part2(inp: String) {
    let nums = inp
        .lines()
        .map(|c| c.parse().unwrap())
        .collect::<Vec<usize>>();
    let mut seen = HashSet::new();
    let mut seen2: HashMap<usize, usize> = HashMap::new();
    for x in nums.iter() {
        let key = 2020 - x;
        seen.iter().for_each(|s| {
            seen2.insert(x + s, x * s);
        });
        if seen2.contains_key(&key) {
            println!("{}", seen2.get(&key).unwrap() * x);
            return;
        }
        seen.insert(*x);
    }
}
