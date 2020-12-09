pub fn find_first_invalid(encrypted: Vec<u64>) -> u64 {
    let mut found = 0;
    for i in 25..1000 {
        if found == 0 {
            for j in encrypted[i - 25..i].iter() {
                if j < &encrypted[i] && encrypted[i - 25..i].contains(&(&encrypted[i] - j)) {
                    found = *j;
                }
            }
            if found == 0 {
                found = encrypted[i];
            } else {
                found = 0;
            }
        }
    }
    found
}

pub fn find_weakness(encrypted: Vec<u64>) -> u64 {
    let mut rev = encrypted.clone();
    rev.reverse();
    let first_invalid = find_first_invalid(encrypted);

    for (i, x) in rev.iter().enumerate() {
        if x < &first_invalid {
            for j in i + 1..1000 {
                let temp_sum = (rev[i..j + 1]).iter().sum::<u64>();
                if temp_sum == first_invalid {
                    let max = &rev[i..j + 1].iter().max().unwrap();
                    let min = &rev[i..j + 1].iter().min().unwrap();

                    println!("{}, {}, {}", max, min, *max + *min);
                    panic!();
                }
            }
        }
    }
    0
}

pub fn part1(inp: String) {
    let encrypted: Vec<u64> = inp.split("\n").map(|i| i.parse().unwrap()).collect();
    println!("{}", find_first_invalid(encrypted));
}

pub fn part2(inp: String) {
    let encrypted: Vec<u64> = inp.split("\n").map(|i| i.parse().unwrap()).collect();
    println!("{}", find_weakness(encrypted));
}
