use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct CustomForm<'a> {
    yes: Vec<&'a str>,
}

impl<'a> From<&'a str> for CustomForm<'a> {
    fn from(s: &'a str) -> Self {
        CustomForm::new(s)
    }
}

impl<'a> CustomForm<'a> {
    pub fn new(s: &'a str) -> Self {
        let mut yes = Vec::new();
        for c in s.split('\n') {
            yes.push(c);
        }
        CustomForm { yes }
    }

    pub fn group_any_yes(&self) -> usize {
        let mut dedupe: HashSet<char> = HashSet::new();
        for m in self.yes.iter() {
            for c in m.chars() {
                dedupe.insert(c);
            }
        }
        dedupe.len()
    }

    pub fn group_all_yes(&self) -> usize {
        let groups: Vec<HashSet<char>> = self.yes.iter().map(|a| a.chars().collect()).collect();

        let mut iter = groups.iter();
        iter.next()
            .map(|start| {
                iter.fold(start.clone(), |all, next| {
                    all.intersection(next).into_iter().cloned().collect()
                })
            })
            .unwrap()
            .len()
    }
}

pub fn part1(inp: String) {
    println!(
        "{}",
        inp.split("\n\n")
            .map(|f| f.into())
            .fold(0, |acc, f: CustomForm| acc + f.group_any_yes())
    );
}

pub fn part2(inp: String) {
    println!(
        "{}",
        inp.split("\n\n")
            .map(|f| f.into())
            .fold(0, |acc, f: CustomForm| acc + f.group_all_yes())
    );
}
