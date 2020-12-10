use std::iter::repeat;

pub fn find_first_invalid(preamble: usize, encrypted: Vec<u64>) -> u64 {
    (preamble..encrypted.len())
        .filter(|i| {
            !repeat(i)
                .zip(encrypted[i - preamble..*i].iter())
                .any(|(i, j)| {
                    j < &encrypted[*i]
                        && encrypted[i - preamble..*i].contains(&(&encrypted[*i] - j))
                })
        })
        .map(|i| encrypted[i])
        .next()
        .unwrap()
}

pub fn find_weakness(preamble: usize, encrypted: Vec<u64>) -> u64 {
    let mut rev = encrypted.clone();
    rev.reverse();
    let first_invalid = find_first_invalid(preamble, encrypted);
    let mut found = 0;

    for (i, x) in rev.iter().enumerate() {
        if found == 0 && x < &first_invalid {
            for j in i + 1..rev.len() {
                let temp_sum = (rev[i..j + 1]).iter().sum::<u64>();
                if found == 0 && temp_sum == first_invalid {
                    let max = &rev[i..j + 1].iter().max().unwrap();
                    let min = &rev[i..j + 1].iter().min().unwrap();
                    found = *max + *min;
                }
            }
        }
    }
    found
}

pub fn part1(inp: String) {
    let encrypted: Vec<u64> = inp.split("\n").map(|i| i.parse().unwrap()).collect();
    println!("{}", find_first_invalid(25, encrypted));
}

pub fn part2(inp: String) {
    let encrypted: Vec<u64> = inp.split("\n").map(|i| i.parse().unwrap()).collect();
    println!("{}", find_weakness(25, encrypted));
}
