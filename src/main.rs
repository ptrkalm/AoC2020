#![allow(dead_code)]
macro_rules! solve {
    ($day:ident) => {
        println!(stringify!($day));
        println!("Part 1: {:?}",   $day::part1($day::INPUT));
        println!("Part 2: {:?}\n", $day::part2($day::INPUT));
    };
}

mod day01; mod day02; mod day03; mod day04; mod day05;
mod day06; mod day07; mod day08; mod day09; mod day10;
mod day11; mod day12; mod day13; mod day14; mod day15;
mod day16; mod day17; mod day18;

fn main() {
   /*
   solve!(day01); solve!(day02); solve!(day03); solve!(day04); solve!(day05);
   solve!(day06); solve!(day07); solve!(day08); solve!(day09); solve!(day10);
   solve!(day11); solve!(day12); solve!(day13); solve!(day14); solve!(day15);
   solve!(day16); solve!(day17); 
   */
   solve!(day18);
}

