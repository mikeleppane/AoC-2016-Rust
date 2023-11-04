use aoc_2016_rust::{run_solution, Runner};
use day01::Day01;
use day02::Day02;

use crate::solutions::day03::Day03;
use crate::solutions::day04::Day04;
use crate::solutions::day05::Day05;
use crate::solutions::day06::Day06;
use crate::Selector;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

pub fn run(which: Selector) {
    let mut day01 = Day01::new();
    let mut day02 = Day02::new();
    let mut day03 = Day03::new();
    let mut day04 = Day04::new();
    let mut day05 = Day05::new();
    let mut day06 = Day06::new();
    let mut day07 = day07::Day07::new();

    let mut days: Vec<&mut dyn Runner> = vec![
        &mut day01, &mut day02, &mut day03, &mut day04, &mut day05, &mut day06, &mut day07,
    ];

    match which {
        Selector::Last => {
            let last = days.len() - 1;
            if let Some(d) = days.get_mut(last) {
                run_solution(*d);
            }
        }
        Selector::All => {
            for d in days {
                run_solution(d);
            }
        }
        Selector::One(num) => {
            if let Some(d) = days.get_mut(num - 1) {
                run_solution(*d);
            }
        }
    }
}
