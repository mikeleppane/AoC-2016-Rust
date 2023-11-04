use aoc_2016_rust::{output, read_lines, Runner};

const INPUT: &str = "input/day07.txt";

#[derive(Default)]
pub struct Day07 {
    ips: Vec<String>,
}

impl Day07 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day07 {
    fn name(&self) -> (usize, usize) {
        (2016, 7)
    }

    fn parse(&mut self) {
        self.ips = read_lines(INPUT);
    }

    fn part1(&mut self) -> Vec<String> {
        let mut count = 0;
        for ip in &self.ips {
            if supports_tls(ip) {
                count += 1;
            }
        }
        output(count)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut count = 0;
        for ip in &self.ips {
            if supports_ssl(ip) {
                count += 1;
            }
        }
        output(count)
    }
}

// ---------------------------------------------------

fn supports_tls(ip: &str) -> bool {
    let mut in_brackets = false;
    let mut has_abba = false;
    let mut has_abba_in_brackets = false;

    for (i, c) in ip.chars().enumerate() {
        if c == '[' {
            in_brackets = true;
        } else if c == ']' {
            in_brackets = false;
        } else if i >= 3
            && c == ip.chars().nth(i - 3).unwrap()
            && c != ip.chars().nth(i - 2).unwrap()
            && ip.chars().nth(i - 1).unwrap() == ip.chars().nth(i - 2).unwrap()
        {
            if in_brackets {
                has_abba_in_brackets = true;
            } else {
                has_abba = true;
            }
        }
    }

    has_abba && !has_abba_in_brackets
}

fn supports_ssl(ip: &str) -> bool {
    let mut in_brackets = false;
    let mut abas = Vec::new();
    let mut babs = Vec::new();

    for (i, c) in ip.chars().enumerate() {
        if c == '[' {
            in_brackets = true;
        } else if c == ']' {
            in_brackets = false;
        } else if i >= 2
            && c == ip.chars().nth(i - 2).unwrap()
            && c != ip.chars().nth(i - 1).unwrap()
        {
            if in_brackets {
                babs.push(format!(
                    "{}{}{}",
                    ip.chars().nth(i - 1).unwrap(),
                    c,
                    ip.chars().nth(i - 1).unwrap()
                ));
            } else {
                abas.push(format!("{}{}{}", c, ip.chars().nth(i - 1).unwrap(), c));
            }
        }
    }
    for aba in &abas {
        for bab in &babs {
            if aba == bab {
                return true;
            }
        }
    }

    false
}

// --------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let mut day07 = Day07::new();
        day07.parse();
        let output = day07.part1();
        assert_eq!(output[0], "118")
    }

    #[test]
    fn part2_works() {
        let mut day07 = Day07::new();
        day07.parse();
        let output = day07.part2();
        assert_eq!(output[0], "260")
    }
}
