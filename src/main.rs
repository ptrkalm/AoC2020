macro_rules! solve {
    ($day:ident) => {
        println!(stringify!($day));
        println!("Part 1: {}",   $day::part1($day::INPUT));
        println!("Part 2: {}\n", $day::part2($day::INPUT));
    };
}

mod day1; mod day2; mod day3; mod day4; mod day5;

fn main() {
    solve!(day1); solve!(day2); solve!(day3); solve!(day4); solve!(day5);
}

