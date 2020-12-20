use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

use regex::Regex;

#[derive(Debug)]
pub struct Ticket {
    fields: Vec<usize>,
}

impl Ticket {
    pub fn from(inp: &str) -> Self {
        Ticket {
            fields: inp.split(',').map(|n| n.parse().unwrap()).collect(),
        }
    }

    pub fn is_valid(&self, rules: &[(RangeInclusive<usize>, RangeInclusive<usize>)]) -> bool {
        !self.fields.iter().any(|num| {
            rules
                .iter()
                .all(|(rule1, rule2)| !rule1.contains(num) && !rule2.contains(num))
        })
    }
}

pub fn get_rules(inp: &str) -> Vec<RangeInclusive<usize>> {
    let mut rules = Vec::new();
    lazy_static! {
        static ref RULE_RE: Regex = Regex::new(r"^.*: (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    }
    for line in inp.lines() {
        if let Some(r) = RULE_RE.captures(line) {
            match (
                r.get(1).unwrap().as_str().parse(),
                r.get(2).unwrap().as_str().parse(),
                r.get(3).unwrap().as_str().parse(),
                r.get(4).unwrap().as_str().parse(),
            ) {
                (Ok(a), Ok(b), Ok(c), Ok(d)) => {
                    rules.push(a..=b);
                    rules.push(c..=d);
                }
                _ => panic!(),
            }
        }
    }
    rules
}

pub fn get_rules_part2(inp: &str) -> Vec<(RangeInclusive<usize>, RangeInclusive<usize>)> {
    let mut rules = Vec::new();
    lazy_static! {
        static ref RULE_RE: Regex = Regex::new(r"^.*: (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    }
    for line in inp.lines() {
        if let Some(r) = RULE_RE.captures(line) {
            match (
                r.get(1).unwrap().as_str().parse(),
                r.get(2).unwrap().as_str().parse(),
                r.get(3).unwrap().as_str().parse(),
                r.get(4).unwrap().as_str().parse(),
            ) {
                (Ok(a), Ok(b), Ok(c), Ok(d)) => {
                    rules.push((a..=b, c..=d));
                }
                _ => panic!(),
            }
        }
    }
    rules
}

pub fn part1(inp: String) {
    let sections: Vec<&str> = inp.split("\n\n").collect();
    let rules = get_rules(sections[0]);
    println!(
        "{:?}",
        sections[2]
            .lines()
            .filter(|line| !line.starts_with("nearby"))
            .flat_map(|line| line.split(','))
            .map(|num| num.parse().unwrap())
            .filter(|num| rules.iter().all(|rule| !rule.contains(num)))
            .sum::<usize>()
    );
}

pub fn part2(inp: String) {
    let sections: Vec<&str> = inp.split("\n\n").collect();
    let rules = get_rules_part2(sections[0]);
    let valid = sections[2]
        .lines()
        .filter(|line| !line.starts_with("nearby"))
        .map(Ticket::from)
        .filter(|t| t.is_valid(&rules))
        .map(|t| t.fields)
        .collect::<Vec<Vec<usize>>>();
    let mut flipped = Vec::new();
    for pos in 0..valid[0].len() {
        let pos_vec = valid.iter().map(|v| v[pos]).collect::<Vec<usize>>();
        flipped.push(pos_vec);
    }

    let mut matched_rules = Vec::new();
    for (i, test) in flipped.iter().enumerate() {
        matched_rules.push((
            i,
            rules
                .clone()
                .into_iter()
                .enumerate()
                //.inspect(|r| println!("{:?}", r))
                .filter_map(|(i, (rule1, rule2))| {
                    if test.iter().all(|n| rule1.contains(n) || rule2.contains(n)) {
                        Some(i)
                    } else {
                        None
                    }
                })
                .collect::<Vec<usize>>(),
        ));
    }
    matched_rules.sort_by(|matches0, matches1| matches0.1.len().cmp(&matches1.1.len()));

    let mut seen = HashSet::new();
    let mut results: HashMap<usize, usize> = HashMap::new();

    for (i, m) in matched_rules {
        let hs = m.into_iter().collect::<HashSet<usize>>();
        let seen_clone = seen.clone();
        let t = hs.difference(&seen_clone).next().unwrap();
        seen.insert(*t);
        results.insert(*t, i);
    }

    let mine = sections[1]
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut ans = 1;
    for i in 0..6 {
        ans *= mine[*results.get(&i).unwrap()];
    }

    println!("{:?}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_rules() {
        assert_eq!(
            get_rules("class: 1-3 or 5-7\nrow: 6-11 or 33-44\nseat: 13-40 or 45-50"),
            vec![(1..=3), (5..=7), (6..=11), (33..=44), (13..=40), (45..=50)]
        );
    }
}
