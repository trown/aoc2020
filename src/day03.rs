pub fn get_trees(tree_map: &str, rise: usize, run: usize) -> usize {
    tree_map
        .lines()
        .enumerate()
        .filter(|(i, _)| i % rise == 0)
        .filter(|(i, line)| line.chars().nth((run * i / rise) % line.len()) == Some('#'))
        .count()
}

pub fn part1(inp: String) {
    println!("{}", get_trees(&inp, 1, 3));
}

pub fn part2(inp: String) {
    println!(
        "{}",
        get_trees(&inp, 1, 1)
            * get_trees(&inp, 1, 3)
            * get_trees(&inp, 1, 5)
            * get_trees(&inp, 1, 7)
            * get_trees(&inp, 2, 1)
    );
}
