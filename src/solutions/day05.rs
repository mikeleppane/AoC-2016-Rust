use md5::Digest;

use aoc_2016_rust::{output, read_lines, Runner};

const INPUT: &str = "input/day05.txt";

pub struct Day05 {
    input: String,
    digest: Vec<Digest>,
    index: usize,
}

impl Day05 {
    pub fn new() -> Self {
        Self {
            input: String::default(),
            digest: vec![],
            index: 0,
        }
    }
}

impl Runner for Day05 {
    fn name(&self) -> (usize, usize) {
        (2016, 5)
    }

    fn parse(&mut self) {
        self.input = read_lines(INPUT).first().unwrap().to_string();
    }

    fn part1(&mut self) -> Vec<String> {
        let mut result = "".to_string();
        while let Some(d) = self.next() {
            let idx = d[2] & 0xf;
            if idx < 10 {
                result.push((b'0' + idx) as char);
            } else {
                result.push((b'a' + idx - 10) as char);
            }
            self.digest.push(d);
            if result.len() == 8 {
                break;
            }
        }
        output(result)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut ary = [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '];
        let mut count = 0;
        for d in self.digest.iter() {
            let idx = (d[2] & 0xf) as usize;
            if idx >= 8 {
                continue;
            }

            let val = (d[3] & 0xf0) >> 4;
            if ary[idx] == ' ' {
                ary[idx] = val_to_char(val);
                count += 1;
            }
        }

        while count < 8 {
            let d = self.next().unwrap();
            let idx = (d[2] & 0xf) as usize;
            if idx >= 8 {
                continue;
            }

            let val = (d[3] & 0xf0) >> 4;
            if ary[idx] == ' ' {
                ary[idx] = val_to_char(val);
                count += 1;
            }
        }

        let result: String = ary.iter().collect();
        output(result)
    }
}

impl Iterator for Day05 {
    type Item = Digest;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let digest = md5::compute(format!("{}{}", self.input, self.index));
            self.index += 1;

            if digest[0] == 0 && digest[1] == 0 && digest[2] & 0xf0 == 0 {
                return Some(digest);
            }
        }
    }
}

fn val_to_char(v: u8) -> char {
    if v < 10 {
        (b'0' + v) as char
    } else {
        (b'a' + v - 10) as char
    }
}

// ---------------------------------------------------

// --------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let mut day05 = Day05::new();
        day05.parse();
        let output = day05.part1();
        assert_eq!(output[0], "801b56a7")
    }

    #[test]
    fn part2_works() {}
}
