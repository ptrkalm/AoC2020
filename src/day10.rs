pub fn part1(input: &str) -> u128 {
    let jolts = parse(input);
    let mut ones   = 0;
    let mut threes = 0;
    for i in 1..jolts.len() {
        match jolts[i] - jolts[i-1] {
            1 => ones   += 1,
            3 => threes += 1,
            _ => {}
        }
    }
    ones * threes
}

pub fn part2(input: &str) -> u128 {
    let jolts = parse(input);
    calc(0, &jolts, &mut HashMap::new())
}

use std::collections::HashMap;
fn calc(i: usize, jolts: &Vec<u128>, cache: &mut HashMap<u128, u128>) -> u128 {
    match cache.get(&jolts[i]) {
        Some(x) => *x,
        None    => {
            if i == jolts.len() - 1 {
                cache.insert(jolts[i], 1);
                return 1;
            }
            let mut res = 0;
            if i + 1 < jolts.len() && (jolts[i + 1] - jolts[i] <= 3) {
                res += calc(i + 1, jolts, cache);
            }
            if i + 2 < jolts.len() && (jolts[i + 2] - jolts[i] <= 3) {
                res += calc(i + 2, jolts, cache);
            }
            if i + 3 < jolts.len() && (jolts[i + 3] - jolts[i] <= 3) {
                res += calc(i + 3, jolts, cache);
            }
            cache.insert(jolts[i], res);
            res
        }
    }
}

fn parse(input: &str) -> Vec<u128> {
    let mut jolts = input
    .split_whitespace()
    .map(|x| x.parse::<u128>().unwrap())
    .collect::<Vec<u128>>();
    jolts.push(0);
    jolts.sort();
    jolts.push(jolts[jolts.len() - 1] + 3);
    
    jolts
}

pub const INPUT: &str = 
"35
111
135
32
150
5
106
154
41
7
27
117
109
63
64
21
138
98
40
71
144
13
66
48
12
55
119
103
54
78
65
112
39
128
53
140
77
34
28
81
151
125
85
124
2
99
131
59
60
6
94
33
42
93
14
141
92
38
104
9
29
100
52
19
147
49
74
70
84
113
120
91
97
17
45
139
90
116
149
129
87
69
20
24
148
18
58
123
76
118
130
132
75
110
105
1
8
86
";