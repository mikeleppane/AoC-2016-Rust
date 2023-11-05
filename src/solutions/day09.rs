use aoc_2016_rust::{output, read_lines, Runner};

const INPUT: &str = "input/day09.txt";

#[derive(Default)]
pub struct Day09 {
    input: String,
}

impl Day09 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day09 {
    fn name(&self) -> (usize, usize) {
        (2016, 9)
    }

    fn parse(&mut self) {
        let lines = read_lines(INPUT);
        self.input = lines.get(0).unwrap().to_string();
    }

    fn part1(&mut self) -> Vec<String> {
        output(self.input.decompress().len())
    }

    fn part2(&mut self) -> Vec<String> {
        output(self.input.explode())
    }
}

// ---------------------------------------------------

trait Decompress {
    fn decompress(&self) -> String;
}

impl Decompress for str {
    fn decompress(&self) -> String {
        let mut output = String::new();
        let mut chars = self.chars();
        while let Some(c) = chars.next() {
            if c == '(' {
                let mut marker = String::new();
                for c in chars.by_ref() {
                    if c == ')' {
                        break;
                    }
                    marker.push(c);
                }
                let mut parts = marker.split('x');
                let len = parts.next().unwrap().parse::<usize>().unwrap();
                let times = parts.next().unwrap().parse::<usize>().unwrap();
                let mut repeat = String::new();
                for _ in 0..len {
                    repeat.push(chars.next().unwrap());
                }
                for _ in 0..times {
                    output.push_str(&repeat);
                }
            } else {
                output.push(c);
            }
        }
        output
    }
}

trait Explode {
    fn explode(&self) -> usize;
}

impl Explode for String {
    fn explode(&self) -> usize {
        let mut count = 0;
        let mut chars = self.chars();
        while let Some(c) = chars.next() {
            if c == '(' {
                let mut marker = String::new();
                for c in chars.by_ref() {
                    if c == ')' {
                        break;
                    }
                    marker.push(c);
                }
                let mut parts = marker.split('x');
                let len = parts.next().unwrap().parse::<usize>().unwrap();
                let times = parts.next().unwrap().parse::<usize>().unwrap();
                let mut repeat = String::new();
                for _ in 0..len {
                    repeat.push(chars.next().unwrap());
                }
                count += times * repeat.explode();
            } else {
                count += 1;
            }
        }
        count
    }
}

// --------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let mut day09 = Day09::new();
        day09.parse();
        let output = day09.part1();
        assert_eq!(output[0], "97714")
    }

    #[test]
    fn part2_works() {
        let mut day09 = Day09::new();
        day09.parse();
        let output = day09.part2();
        assert_eq!(output[0], "10762972461")
    }
}
