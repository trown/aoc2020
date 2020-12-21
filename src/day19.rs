// #[derive(Debug)]
// pub enum Rule {
//     Value(char),
//     RuleRef(Vec<Rule>),
//     Or(Vec<Rule>, Vec<Rule>),
// }


pub fn part1(inp: String) {
    println!("{}", inp);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INP: &'static str = r#"0: 4 1 5\n1: 2 3 | 3 2\n2: 4 4 | 5 5\n3: 4 5 | 5 4\n4: "a"\n5: "b"\n\nababbb\nbababa\nabbbab\naaabbb\naaaabbb"#;

    #[test]
    fn test_part1() {
        println!("{}", TEST_INP);
        assert!(false == true);
    }
}
