use std::cmp::Ordering;

#[derive(Debug, Eq)]
pub struct BoardingPass<'a> {
    pass: &'a str,
}

impl<'a> From<&'a str> for BoardingPass<'a> {
    fn from(s: &'a str) -> Self {
        BoardingPass::new(s)
    }
}

impl<'a> BoardingPass<'a> {
    pub fn new(s: &'a str) -> Self {
        BoardingPass { pass: s }
    }

    pub fn get_id(&self) -> usize {
        let row_string = &self.pass[..7];
        let row = usize::from_str_radix(
            row_string
                .chars()
                .map(|l| match l {
                    'B' => '1',
                    'F' => '0',
                    _ => '0',
                })
                .collect::<String>()
                .as_ref(),
            2,
        )
        .unwrap();
        let col_string = &self.pass[7..];
        let col = usize::from_str_radix(
            col_string
                .chars()
                .map(|l| match l {
                    'R' => '1',
                    'L' => '0',
                    _ => '0',
                })
                .collect::<String>()
                .as_ref(),
            2,
        )
        .unwrap();
        row * 8 + col
    }
}

impl<'a> Ord for BoardingPass<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_id().cmp(&other.get_id())
    }
}

impl<'a> PartialOrd for BoardingPass<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for BoardingPass<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

pub fn part1(inp: String) {
    println!(
        "{:?}",
        inp.lines()
            .map(|p| BoardingPass::from(p))
            .max()
            .unwrap()
            .get_id()
    );
}

pub fn part2(inp: String) {
    let mut pass_list = inp
        .lines()
        .map(|p| BoardingPass::from(p))
        .collect::<Vec<BoardingPass>>();
    pass_list.sort();
    let first = pass_list[0].get_id();
    if let Some((last_good, _)) = pass_list
        .iter()
        .enumerate()
        .filter(|(i, p)| p.get_id() - first == *i)
        .last()
    {
        println!("{:?}", last_good + first + 1);
    }
}
