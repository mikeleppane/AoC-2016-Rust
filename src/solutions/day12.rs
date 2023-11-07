use aoc_2016_rust::{output, read_lines, Runner};

const INPUT: &str = "input/day12.txt";

#[derive(Debug, Default)]
pub struct Day12 {
    vm: Machine,
}

impl Day12 {
    pub fn new() -> Self {
        Self {
            vm: Machine::default(),
        }
    }
}

impl Runner for Day12 {
    fn name(&self) -> (usize, usize) {
        (2016, 12)
    }

    fn parse(&mut self) {
        let lines = read_lines(INPUT);
        for line in lines {
            self.vm.prog.push(Instruction::parse(&line));
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let a = self.vm.run(0);
        output(a)
    }

    fn part2(&mut self) -> Vec<String> {
        let a = self.vm.run(1);
        output(a)
    }
}

// ---------------------------------------------------

#[derive(Debug, Default)]
struct Machine {
    prog: Vec<Instruction>,
}

impl Machine {
    fn run(&mut self, initial_c: i32) -> i32 {
        let mut ip = 0;
        let mut reg: [i32; 4] = [0, 0, initial_c, 0];
        while ip < self.prog.len() {
            let old_ip = ip;
            ip += 1;
            match &self.prog[old_ip] {
                Instruction::Cpy(Source::Reg(r), dest) => {
                    reg[*dest as usize] = reg[*r as usize];
                }
                Instruction::Cpy(Source::Value(v), dest) => {
                    reg[*dest as usize] = *v;
                }

                Instruction::Inc(r) => {
                    reg[*r as usize] += 1;
                }
                Instruction::Dec(r) => {
                    reg[*r as usize] -= 1;
                }
                Instruction::Jnz(src, dst) => {
                    let val = match src {
                        Source::Reg(r) => reg[*r as usize],
                        Source::Value(v) => *v,
                    };
                    if val != 0 {
                        if *dst < 0 {
                            ip = (old_ip as i32 - (-*dst)) as usize;
                        } else {
                            ip = (old_ip as i32 + *dst) as usize;
                        }
                    }
                }
            }
        }
        reg[0]
    }
}

#[derive(Debug)]
enum Instruction {
    Cpy(Source, Register),
    Inc(Register),
    Dec(Register),
    Jnz(Source, i32),
}

impl Instruction {
    fn parse(s: &str) -> Self {
        let mut parts = s.split_whitespace();
        let op = parts.next().unwrap();
        match op {
            "cpy" => {
                let src = parts.next().unwrap();
                let dst = parts.next().unwrap();
                let src = Source::parse(src);
                let dst = Register::parse(dst).unwrap();
                Instruction::Cpy(src, dst)
            }
            "inc" => {
                let dst = parts.next().unwrap();
                let dst = Register::parse(dst).unwrap();
                Instruction::Inc(dst)
            }
            "dec" => {
                let dst = parts.next().unwrap();
                let dst = Register::parse(dst).unwrap();
                Instruction::Dec(dst)
            }
            "jnz" => {
                let src = parts.next().unwrap();
                let dst = parts.next().unwrap();
                let src = Source::parse(src);
                let dst = dst.parse::<i32>().unwrap();
                Instruction::Jnz(src, dst)
            }
            _ => panic!("Unknown instruction: {}", op),
        }
    }
}

#[derive(Debug)]
enum Source {
    Reg(Register),
    Value(i32),
}

impl Source {
    fn parse(s: &str) -> Self {
        match Register::parse(s) {
            Ok(v) => Source::Reg(v),
            Err(_) => Source::Value(s.parse::<i32>().unwrap()),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Register {
    A,
    B,
    C,
    D,
}

impl Register {
    fn parse(s: &str) -> Result<Self, ()> {
        match s {
            "a" => Ok(Register::A),
            "b" => Ok(Register::B),
            "c" => Ok(Register::C),
            "d" => Ok(Register::D),
            _ => Err(()),
        }
    }
}

// --------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let mut day = Day12::new();
        day.parse();
        let output = day.part1();
        assert_eq!(output[0], "318003")
    }

    #[test]
    fn part2_works() {
        let mut day = Day12::new();
        day.parse();
        let output = day.part2();
        assert_eq!(output[0], "9227657")
    }
}
