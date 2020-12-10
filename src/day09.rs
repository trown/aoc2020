pub fn find_first_invalid(preamble: usize, encrypted: Vec<u64>) -> u64 {
    (preamble..encrypted.len())
        .filter_map(|i| {
            if encrypted[i - preamble..i].iter().any(|j| {
                j < &encrypted[i] && encrypted[i - preamble..i].contains(&(&encrypted[i] - j))
            }) {
                None
            } else {
                Some(encrypted[i])
            }
        })
        .next()
        .unwrap()
}

pub fn find_weakness(preamble: usize, encrypted: Vec<u64>) -> u64 {
    let mut rev = encrypted.clone();
    rev.reverse();
    let first_invalid = find_first_invalid(preamble, encrypted);
    rev.iter()
        .enumerate()
        .filter(|(_, x)| *x < &first_invalid)
        .filter_map(|(i, _)| {
            (i + 1..rev.len())
                .filter(|j| first_invalid == (rev[i..j + 1]).iter().sum::<u64>())
                .map(|j| {
                    let max = &rev[i..j + 1].iter().max().unwrap();
                    let min = &rev[i..j + 1].iter().min().unwrap();
                    *max + *min
                })
                .next()
        })
        .next()
        .unwrap()
}

pub fn part1(inp: String) {
    let encrypted: Vec<u64> = inp.split("\n").map(|i| i.parse().unwrap()).collect();
    println!("{}", find_first_invalid(25, encrypted));
}

pub fn part2(inp: String) {
    let encrypted: Vec<u64> = inp.split("\n").map(|i| i.parse().unwrap()).collect();
    println!("{}", find_weakness(25, encrypted));
}
