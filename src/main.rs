#![allow(dead_code)]
macro_rules! solve {
    ($day:ident) => {
        println!(stringify!($day));
        println!("Part 1: {:?}",   $day::part1($day::INPUT));
        println!("Part 2: {:?}\n", $day::part2($day::INPUT));
    };
}

mod day1;  mod day2;  mod day3;  mod day4;  mod day5;
mod day6;  mod day7;  mod day8;  mod day9;  mod day10;
mod day11; mod day12; mod day13;

fn main() {
   solve!(day1);  solve!(day2);  solve!(day3);  solve!(day4); solve!(day5);
   solve!(day6);  solve!(day7);  solve!(day8);  solve!(day9); solve!(day10);
   solve!(day11); solve!(day12); solve!(day13); 
}

