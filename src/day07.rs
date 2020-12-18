use std::collections::HashMap;

use regex::Regex;

#[derive(Debug, Clone)]
pub struct LuggageRules<'a> {
    rules: HashMap<&'a str, Vec<(usize, &'a str)>>,
}

impl<'a> From<&'a str> for LuggageRules<'a> {
    fn from(s: &'a str) -> Self {
        LuggageRules::new(s)
    }
}

impl<'a> LuggageRules<'a> {
    pub fn new(s: &'a str) -> Self {
        lazy_static! {
            static ref LR_RE: Regex = Regex::new(r"(\d? ?[a-z]+ [a-z]+)").unwrap();
        }
        let mut rules = HashMap::new();
        for rule in s.split('\n') {
            let mut iter = LR_RE.captures_iter(rule);
            if let Some(k) = iter.next() {
                let k = k.get(0).unwrap().as_str();
                let v: Vec<(usize, &'a str)> = iter
                    .filter(|b| !b.get(0).unwrap().as_str().starts_with(' '))
                    .map(|b| {
                        let bag = b.get(0).unwrap().as_str();
                        (bag[..1].parse::<usize>().unwrap_or(0), &bag[2..])
                    })
                    .collect();
                rules.insert(k, v);
            }
        }
        LuggageRules { rules }
    }

    pub fn can_contain_gold(&self, mut seen: &mut HashMap<&'a str, bool>, color: &'a str) -> bool {
        if let Some(c) = seen.get(color) {
            *c
        } else if let Some(rules) = self.rules.get(color) {
            if format!("{:?}", rules).contains(&"shiny gold") {
                seen.insert(color, true);
                true
            } else {
                let c = rules
                    .iter()
                    .map(|(_, color)| self.can_contain_gold(&mut seen, color))
                    .any(|c| c);
                seen.insert(color, c);
                c
            }
        } else {
            false
        }
    }

    pub fn bags_contained(&self, mut seen: &mut HashMap<&'a str, usize>, color: &'a str) -> usize {
        if let Some(c) = seen.get(color) {
            *c
        } else if let Some(rules) = self.rules.get(color) {
            let c = rules
                .iter()
                .map(|(n, color)| n + n * self.bags_contained(&mut seen, color))
                .sum();
            seen.insert(color, c);
            c
        } else {
            seen.insert(color, 0);
            0
        }
    }
}

pub fn part1(inp: String) {
    let lr = LuggageRules::from(inp.as_str());
    let mut seen: HashMap<&str, bool> = HashMap::new();
    println!(
        "{:?}",
        lr.rules
            .keys()
            .filter(|rule| lr.can_contain_gold(&mut seen, rule))
            .count()
    );
}

pub fn part2(inp: String) {
    let lr = LuggageRules::from(inp.as_str());
    let mut seen: HashMap<&str, usize> = HashMap::new();
    println!("{:?}", lr.bags_contained(&mut seen, "shiny gold"));
}
