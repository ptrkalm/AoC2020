pub fn part1(input: &str) -> i128 {
    let (est, buses) = parse(input);
    let pairs = buses.into_iter().map(|bus| (bus.1, bus.1 * (est / bus.1 + 1))).collect::<Vec<(i128, i128)>>();
    let min = pairs.into_iter().min_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap();
    min.0 * (min.1 - est)
}

pub fn part2(input: &str) -> i128 {
    let (_, buses) = parse(input);
    let rems = buses.clone().into_iter().map(|(off, id)| (id - off as i128) % id).collect::<Vec<i128>>();
    let mods = buses.into_iter().map(|(_, id)| id).collect::<Vec<i128>>();
    crt(rems, mods)
}

fn egcd(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i128, n: i128) -> i128 {
    let (_, x, _) = egcd(x, n);
    (x % n + n) % n
}

fn crt(rems: Vec<i128>, mods: Vec<i128>) -> i128 {
    let prod = mods.iter().product::<i128>();
    let mut sum = 0;

    for i in 0..rems.len() {
        let p = prod / mods[i];
        sum += rems[i] * mod_inv(p, mods[i]) * p
    }

    sum % prod
}

fn parse(input: &str) -> (i128, Vec<(usize, i128)>) {
    let lines = input.lines().collect::<Vec<&str>>();
    let buses = 
    lines[1]
    .split(",")
    .enumerate()
    .map(|(i, x)| (i, x.parse::<i128>().unwrap_or(0)))
    .filter(|&(_, x)| x != 0)
    .collect::<Vec<(usize, i128)>>();

    (lines[0].parse::<i128>().unwrap(), buses)
}   

pub const INPUT: &str = 
"1014511
17,x,x,x,x,x,x,41,x,x,x,x,x,x,x,x,x,643,x,x,x,x,x,x,x,23,x,x,x,x,13,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,29,x,433,x,x,x,x,x,37,x,x,x,x,x,x,x,x,x,x,x,x,19";