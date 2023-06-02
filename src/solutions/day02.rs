use aoc_2016_rust::{output, read_lines, Runner};

const INPUT: &str = "input/day02.txt";

pub struct Day02 {
    lines: Vec<Vec<char>>,
}

impl Day02 {
    pub fn new() -> Self {
        Self { lines: vec![] }
    }
}

impl Runner for Day02 {
    fn name(&self) -> (usize, usize) {
        (2016, 2)
    }

    fn parse(&mut self) {
        for line in read_lines(INPUT) {
            let chars = line.chars().collect::<Vec<char>>();
            self.lines.push(chars);
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut x = 1i32;
        let mut y = 1i32;
        let keypad = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
        let mut answer = String::from("");

        for key in &self.lines {
            for step in key.iter() {
                match step {
                    'U' => {
                        if y > 0 {
                            y -= 1
                        }
                    }
                    'D' => {
                        if y < 2 {
                            y += 1
                        }
                    }
                    'L' => {
                        if x > 0 {
                            x -= 1
                        }
                    }
                    'R' => {
                        if x < 2 {
                            x += 1
                        }
                    }
                    _ => panic!("corrupted input file"),
                }
            }
            answer.push(keypad[(y * 3 + x) as usize])
        }
        output(answer)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut x = 0usize;
        let mut y = 2usize;
        let keypad = [
            '0', '0', '1', '0', '0', '0', '2', '3', '4', '0', '5', '6', '7', '8', '9', '0', 'A',
            'B', 'C', '0', '0', '0', 'D', '0', '0',
        ];
        let mut answer = String::from("");

        for key in &self.lines {
            for step in key.iter() {
                match step {
                    'U' => {
                        if y > 0 && keypad[(y - 1) * 5 + x] != '0' {
                            y -= 1
                        }
                    }
                    'D' => {
                        if y < 4 && keypad[(y + 1) * 5 + x] != '0' {
                            y += 1
                        }
                    }
                    'L' => {
                        if x > 0 && keypad[y * 5 + (x - 1)] != '0' {
                            x -= 1
                        }
                    }
                    'R' => {
                        if x < 4 && keypad[y * 5 + x + 1] != '0' {
                            x += 1
                        }
                    }
                    _ => panic!("corrupted input file"),
                }
            }
            answer.push(keypad[y * 5 + x])
        }
        output(answer)
    }
}

// ---------------------------------------------------

// --------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let mut day01 = Day02::new();
        day01.parse();
        let output = day01.part1();
        assert_eq!(output[0], "78293")
    }

    #[test]
    fn part2_works() {
        let mut day02 = Day02::new();
        day02.parse();
        let output = day02.part2();
        assert_eq!(output[0], "AC8C8")
    }
}
