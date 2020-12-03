pub fn get_trees(tree_map: &Vec<&str>, rise: usize, run: usize) -> usize {
    tree_map
        .iter()
        .enumerate()
        .filter(|(i, _)| i % rise == 0)
        .filter(|(i, line)| line.chars().nth((run * i / rise) % line.len()) == Some('#'))
        .count()
}

pub fn part1(inp: String) {
    let tree_map = inp.lines().collect::<Vec<&str>>();
    println!("{}", get_trees(&tree_map, 1, 3));
}

pub fn part2(inp: String) {
    let tree_map = inp.lines().collect::<Vec<&str>>();
    let slope1 = get_trees(&tree_map, 1, 1);
    let slope2 = get_trees(&tree_map, 1, 3);
    let slope3 = get_trees(&tree_map, 1, 5);
    let slope4 = get_trees(&tree_map, 1, 7);
    let slope5 = get_trees(&tree_map, 2, 1);
    println!("{}", slope1 * slope2 * slope3 * slope4 * slope5);
}
