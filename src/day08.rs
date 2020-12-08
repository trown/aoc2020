use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Program<'a> {
    instructions: Vec<Instruction<'a>>,
    pointer: usize,
    accumulator: i32,
    seen: HashSet<usize>,
    state: ProgramState,
}

#[derive(Clone, Debug)]
pub enum ProgramState {
    Init,
    Complete,
    Killed,
}

impl<'a> Iterator for Program<'a> {
    type Item = Instruction<'a>;

    fn next(&mut self) -> Option<Instruction<'a>> {
        if self.seen.contains(&self.pointer) {
            self.state = ProgramState::Killed;
            None
        } else if self.pointer >= self.instructions.len() - 1 {
            self.state = ProgramState::Complete;
            None
        } else {
            let ins = self.instructions[self.pointer].clone();
            match ins.code {
                "acc" => {
                    self.seen.insert(self.pointer);
                    self.pointer += 1;
                    self.accumulator += ins.arg;
                    Some(self.instructions[self.pointer].clone())
                }
                "jmp" => {
                    self.seen.insert(self.pointer);
                    self.pointer = (self.pointer as i32 + ins.arg) as usize;
                    Some(self.instructions[self.pointer].clone())
                }
                "nop" => {
                    self.seen.insert(self.pointer);
                    self.pointer += 1;
                    Some(self.instructions[self.pointer].clone())
                }
                _ => None,
            }
        }
    }
}

impl<'a> Program<'a> {
    pub fn new(instructions: Vec<Instruction<'a>>) -> Self {
        Program {
            instructions,
            pointer: 0,
            accumulator: 0,
            seen: HashSet::new(),
            state: ProgramState::Init,
        }
    }

    pub fn reset(&mut self) {
        self.accumulator = 0;
        self.seen = HashSet::new();
        self.pointer = 0;
        self.state = ProgramState::Init;
    }

    pub fn find_loop(&mut self) -> i32 {
        self.into_iter().count();
        self.accumulator
    }

    pub fn fix_program(&mut self) -> i32 {
        let mut acc = 0;
        let instructions = self.instructions.clone();
        for (i, v) in instructions.iter().enumerate() {
            if acc == 0 && ["jmp", "nop"].contains(&v.code) {
                self.instructions[i].code = flip(v.code);
                acc = self.find_loop();
                match self.state {
                    ProgramState::Complete => (),
                    _ => {
                        acc = 0;
                        self.instructions[i].code = v.code;
                        self.reset();
                    }
                }
            }
        }
        acc
    }
}

#[derive(Debug, Clone)]
pub struct Instruction<'a> {
    code: &'a str,
    arg: i32,
}

impl<'a> From<&'a str> for Instruction<'a> {
    fn from(s: &'a str) -> Self {
        Instruction::new(s)
    }
}

impl<'a> Instruction<'a> {
    pub fn new(s: &'a str) -> Self {
        let v: Vec<_> = s.split(" ").collect();
        Instruction {
            code: v[0],
            arg: v[1].parse().unwrap(),
        }
    }
}

pub fn flip(code: &str) -> &str {
    match code {
        "jmp" => "nop",
        "nop" => "jmp",
        _ => "nop",
    }
}

pub fn part1(inp: String) {
    let instructions: Vec<Instruction> = inp.split("\n").map(|i| i.into()).collect();
    let mut program = Program::new(instructions);
    println!("{:?}", program.find_loop());
}

pub fn part2(inp: String) {
    let instructions: Vec<Instruction> = inp.split("\n").map(|i| i.into()).collect();
    let mut program = Program::new(instructions);
    println!("{:?}", program.fix_program());
}
