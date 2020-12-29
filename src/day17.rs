use std::collections::HashMap;
const NEIGHBOURS: [[i64; 3]; 26] = [
    //z = 0
    [-1, -1,  0], [0, -1,  0], [1, -1,  0],
    [-1, 0,   0],              [1,  0,  0],
    [-1, 1,   0], [0,  1,  0], [1,  1,  0],

    //z = -1
    [-1, -1, -1], [0, -1, -1], [1, -1, -1],
    [-1, 0,  -1], [0,  0, -1], [1,  0, -1],
    [-1, 1,  -1], [0,  1, -1], [1,  1, -1],

    //z = 1
    [-1, -1,  1], [0, -1,  1], [1, -1,  1],
    [-1,  0,  1], [0,  0,  1], [1,  0,  1],
    [-1,  1,  1], [0,  1,  1], [1,  1,  1],

];

pub fn part1(input: &str) -> i64 {
    let mut grid = parse3d(input);
    simulate3d(&mut grid);
    count3d(&grid)
}

pub fn part2(input: &str) -> i64 {
    let mut grid = parse4d(input);
    simulate4d(&mut grid);
    count4d(&grid)
}

fn simulate3d(grid: &mut HashMap<(i64, i64, i64), char>) {
    for _ in 0..6 {
        let mut count_grid: HashMap<(i64, i64, i64), i64> = HashMap::new();
        for cord in grid.keys() {
            if let Some(_) = count_grid.get(cord) {} else {
                count_grid.insert(*cord, 0);
            }
            if *grid.get(cord).unwrap() == '#' {
                for x in -1..=1 {
                for y in -1..=1 {
                for z in -1..=1 {
                    if x == 0 && y == 0 && z == 0 {continue}
                    let neighbour = (cord.0 + x, cord.1 + y, cord.2 + z);
                    
                    match count_grid.insert(neighbour, 1) {
                        None    => {},
                        Some(n) => {count_grid.insert(neighbour, n + 1);}
                    };
                }}}
            }
        }
        for cord in count_grid.keys() {
            match (*grid.get(cord).unwrap_or(&'.') == '#', count_grid.get(cord).unwrap()) {
                (true,  2) |
                (true,  3) |
                (false, 3) => grid.insert(*cord, '#'),
                _          => grid.insert(*cord, '.')
            };
        }
    }
}

fn simulate4d(grid: &mut HashMap<(i64, i64, i64, i64), char>) {
    for _ in 0..6 {
        let mut count_grid: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();
        for cord in grid.keys() {
            if let Some(_) = count_grid.get(cord) {} else {
                count_grid.insert(*cord, 0);
            }
            if *grid.get(cord).unwrap() == '#' {
                for x in -1..=1 {
                for y in -1..=1 {
                for z in -1..=1 {
                for w in -1..=1 {
                    if x == 0 && y == 0 && z == 0 && w == 0 {continue}
                    let neighbour = (cord.0 + x, cord.1 + y, cord.2 + z, cord.3 + w);
                    
                    match count_grid.insert(neighbour, 1) {
                        None    => {},
                        Some(n) => {count_grid.insert(neighbour, n + 1);}
                    };
                }}}}
            }
        }
        for cord in count_grid.keys() {
            match (*grid.get(cord).unwrap_or(&'.') == '#', count_grid.get(cord).unwrap()) {
                (true,  2) |
                (true,  3) |
                (false, 3) => grid.insert(*cord, '#'),
                _          => grid.insert(*cord, '.')
            };
        }
    }
}

fn count3d(grid: &HashMap<(i64, i64, i64), char>) -> i64 {
    let mut count = 0;
    for c in grid.values() {
        if c == &'#' {
            count += 1;
        } 
    }
    count
}

fn count4d(grid: &HashMap<(i64, i64, i64, i64), char>) -> i64 {
    let mut count = 0;
    for c in grid.values() {
        if c == &'#' {
            count += 1;
        } 
    }
    count
}

fn parse3d(input: &str) -> HashMap<(i64, i64, i64), char> {
    let grid: Vec<(usize, Vec<(usize, char)>)> =
    input
    .lines()
    .map(|line|
        line
        .chars()
        .enumerate()
        .collect::<Vec<(usize, char)>>()
    )
    .enumerate()
    .collect();

    let mut map: HashMap<(i64, i64, i64), char> = HashMap::new();

    for row in &grid {
        for col in &row.1 {
            map.insert((row.0 as i64, col.0 as i64, 0), col.1);
        }
    }
    map
}

fn parse4d(input: &str) -> HashMap<(i64, i64, i64, i64), char> {
    let grid: Vec<(usize, Vec<(usize, char)>)> =
    input
    .lines()
    .map(|line|
        line
        .chars()
        .enumerate()
        .collect::<Vec<(usize, char)>>()
    )
    .enumerate()
    .collect();

    let mut map: HashMap<(i64, i64, i64, i64), char> = HashMap::new();

    for row in &grid {
        for col in &row.1 {
            map.insert((row.0 as i64, col.0 as i64, 0, 0), col.1);
        }
    }
    map
}

pub const INPUT: &str =
"...#...#
#######.
....###.
.#..#...
#.#.....
.##.....
#.####..
#....##.";