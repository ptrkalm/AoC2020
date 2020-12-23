use std::collections::HashMap;

pub fn part1(input: &str) -> i64 {
    let starting = parse(input);
    nth(starting, 2020)
}

pub fn part2(input: &str) -> i64 {
    let starting = parse(input);
    nth(starting, 30000000)
}

fn nth(starting: Vec<i64>, n: usize) -> i64 {
    let mut map: HashMap<i64, usize> = HashMap::new();
    for number in starting.iter().enumerate() {
        map.insert(*number.1, number.0 + 1);
    }
    let mut next = 0;
    let mut count = starting.len() + 1;
    while count < n {
        let repeated = map.get(&next);
        match repeated {
            Some(value) => {
                let new = (count - value) as i64;
                map.insert(next, count);
                next = new;
            },
            None => {
                map.insert(next, count);
                next = 0
            }
        };
        
        count += 1;
    }
    next
}

fn parse(input: &str) -> Vec<i64> {
    input
    .split(",")
    .map(|n| n.parse().unwrap())
    .collect()
}
pub const INPUT: &str = "6,13,1,15,2,0";