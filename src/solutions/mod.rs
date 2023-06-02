use aoc_2016_rust::run_solution;
use day01::Day01;
use day02::Day02;

use crate::{Runner, Selector};

mod day01;
mod day02;

pub fn run(which: Selector) {
    let mut day01 = Day01::new();
    let mut day02 = Day02::new();

    let mut days: Vec<&mut dyn Runner> = vec![&mut day01, &mut day02];

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
