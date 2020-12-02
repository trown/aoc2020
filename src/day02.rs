use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct PassEntry {
    rule: PassRule,
    pass: String,
}

impl FromStr for PassEntry {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pl: Vec<&str> = s.split(' ').collect();
        let nums: Vec<&str> = pl[0].split('-').collect();
        let freq_min: usize = nums[0].parse()?;
        let freq_max: usize = nums[1].parse()?;
        let match_char = pl[1].chars().next().unwrap();
        let pass = pl[2].into();
        Ok(PassEntry {
            rule: PassRule {
                freq_min,
                freq_max,
                match_char,
            },
            pass,
        })
    }
}

impl PassEntry {
    pub fn is_valid(&self) -> bool {
        let match_count = self.pass.matches(self.rule.match_char).count();
        match_count >= self.rule.freq_min && match_count <= self.rule.freq_max
    }

    pub fn is_valid_part2(&self) -> bool {
        let pass_chars: Vec<char> = self.pass.chars().collect();
        let match_pos1 = pass_chars[self.rule.freq_min - 1] == self.rule.match_char;
        let match_pos2 = pass_chars[self.rule.freq_max - 1] == self.rule.match_char;
        (match_pos1 || match_pos2) && !(match_pos1 && match_pos2)
    }
}

#[derive(Debug, Clone)]
pub struct PassRule {
    freq_min: usize,
    freq_max: usize,
    match_char: char,
}

pub fn part1(inp: String) {
    let pass_map = inp
        .lines()
        .map(|e| e.parse::<PassEntry>().unwrap())
        .collect::<Vec<PassEntry>>();
    println!("{}", pass_map.iter().filter(|p| p.is_valid()).count());
}

pub fn part2(inp: String) {
    let pass_map = inp
        .lines()
        .map(|e| e.parse::<PassEntry>().unwrap())
        .collect::<Vec<PassEntry>>();
    println!("{}", pass_map.iter().filter(|p| p.is_valid_part2()).count());
}
