use std::collections::HashMap;

use regex::Regex;

#[derive(Debug, Clone)]
pub struct Program<'a> {
    instructions: Vec<Instruction<'a>>,
}

impl<'a> Program<'a> {
    pub fn new(instructions: Vec<Instruction<'a>>) -> Self {
        Program { instructions }
    }

    pub fn process(self, memory: &mut HashMap<usize, usize>, part: Part) {
        let mut mask = "";
        for instruction in self.instructions.iter() {
            match (instruction, part) {
                (Instruction::Mask(m), _) => {
                    //println!("Mask: {}", m);
                    mask = m;
                }
                (Instruction::Mem((addr, val)), Part::Part1) => {
                    memory.insert(*addr, apply_mask_part1(mask, val));
                }
                (Instruction::Mem((addr, val)), Part::Part2) => {
                    for a in apply_mask_part2(mask, addr) {
                        memory.insert(a, *val);
                    }
                }
            }
        }
    }
}

#[derive(Copy, Clone)]
pub enum Part {
    Part1,
    Part2,
}
pub fn apply_mask_part1(mask: &str, val: &usize) -> usize {
    let mutate = format!("{:036b}", val)
        .chars()
        .zip(mask.chars())
        .map(|(b, m)| match m {
            '0' => '0',
            '1' => '1',
            _ => b,
        })
        .collect::<String>();
    usize::from_str_radix(&mutate, 2).unwrap()
}

pub fn apply_mask_part2(mask: &str, addr: &usize) -> Vec<usize> {
    let base = usize::from_str_radix(
        format!("{:036b}", addr)
            .chars()
            .zip(mask.chars())
            .map(|(b, m)| match m {
                'X' => '0',
                '1' => '1',
                _ => b,
            })
            .collect::<String>()
            .as_str(),
        2,
    )
    .unwrap();
    let mut addrs = vec![base];
    for i in mask
        .chars()
        .rev()
        .enumerate()
        .filter_map(|(i, b)| if b == 'X' { Some(i) } else { None })
    {
        for a in addrs.clone() {
            addrs.push(a + 2_usize.pow(i as u32))
        }
    }
    addrs
}

#[derive(Debug, Clone)]
pub enum Instruction<'a> {
    Mask(&'a str),
    Mem((usize, usize)),
}

impl<'a> Instruction<'a> {
    pub fn from(instruction: &'a str) -> Self {
        match &instruction[..4] {
            "mask" => Instruction::Mask(&instruction[7..]),
            "mem[" => {
                lazy_static! {
                    static ref MEM_RE: Regex = Regex::new(r"^(\d+)] = (\d+)$").unwrap();
                }
                let mi = MEM_RE.captures(&instruction[4..]).unwrap();
                Instruction::Mem((
                    mi[1].parse::<usize>().unwrap(),
                    mi[2].parse::<usize>().unwrap(),
                ))
            }
            _ => {
                panic!()
            }
        }
    }
}

pub fn part1(inp: String) {
    let prog = Program::new(
        inp.split('\n')
            .map(|i| Instruction::from(i))
            .collect::<Vec<Instruction>>(),
    );
    let mut memory = HashMap::new();
    prog.process(&mut memory, Part::Part1);
    println!("{:?}", memory.values().sum::<usize>());
}

pub fn part2(inp: String) {
    let prog = Program::new(
        inp.split('\n')
            .map(|i| Instruction::from(i))
            .collect::<Vec<Instruction>>(),
    );
    let mut memory = HashMap::new();
    prog.process(&mut memory, Part::Part2);
    println!("{:?}", memory.values().sum::<usize>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_mask_part2() {
        assert_eq!(
            apply_mask_part2("000000000000000000000000000000X1001X", &42),
            vec![26, 27, 58, 59]
        );
        assert_eq!(
            apply_mask_part2("00000000000000000000000000000000X0XX", &26),
            vec![16, 17, 18, 19, 24, 25, 26, 27]
        );
    }
}
