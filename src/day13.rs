pub fn find_route(earliest: u64, routes: Vec<u64>) -> u64 {
    if let Some(found) = (earliest..)
        .enumerate()
        .find(|(_, e)| routes.iter().any(|r| e % *r == 0))
    {
        routes.iter().find(|r| found.1 % **r == 0).unwrap() * found.0 as u64
    } else {
        0
    }
}

fn inv_mod(x: i64, p: i64) -> i64 {
    // p must be prime
    (0..p - 2).fold(1, |o, _| (o * x) % p)
}

pub fn find_route2(routes: &mut Vec<(i64, i64)>) -> i64 {
    let prod: i64 = routes.iter().map(|(_, b)| b).product();

    routes
        .iter()
        .map(|&(offset, id)| -offset * (prod / id) * inv_mod(prod / id, id))
        .sum::<i64>()
        .rem_euclid(prod)
}

pub fn part1(inp: String) {
    let mut split = inp.split('\n');
    let earliest = split.next().unwrap().parse::<u64>().unwrap();
    let routes: Vec<u64> = split
        .next()
        .unwrap()
        .split(',')
        .filter(|route| route != &"x")
        .map(|route| route.parse::<u64>().unwrap())
        .collect();
    println!("{:?}, {:?}", earliest, routes);
    println!("{:?}", find_route(earliest, routes));
}

pub fn part2(inp: String) {
    let mut split = inp.split('\n');
    let earliest = split.next().unwrap().parse::<i64>().unwrap();
    let mut routes: Vec<(i64, i64)> = split
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, route)| route != &"x")
        .map(|(i, route)| (i as i64, route.parse::<i64>().unwrap()))
        .collect();
    //let mut routes: Vec<(i64, i64)> = vec![(0, 2), (1, 3), (2, 5)];
    println!("{:?}, {:?}", earliest, routes);
    println!("{:?}", find_route2(&mut routes));
}
