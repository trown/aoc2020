pub fn part1(inp: String) {
  let nums = inp.lines().map(|c| c.parse().unwrap()).collect::<Vec<usize>>();
  for x in nums.iter() {
    for y in nums.iter() {
      if x + y == 2020 {
        println!("{}", x*y);
        return
      }
    }
  }
}

pub fn part2(inp: String) {
  let nums = inp.lines().map(|c| c.parse().unwrap()).collect::<Vec<usize>>();
  for x in nums.iter() {
    for y in nums.iter() {
      for z in nums.iter() {
        if x + y + z == 2020 {
          println!("{}", x*y*z);
          return
        }
      }
    }
  }
}