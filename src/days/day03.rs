use regex::Regex;

pub fn run() {
    let input = std::fs::read_to_string("input/day03.txt").unwrap();
    println!("Part1 Result: {}", part_1(&input));
    println!("Part2 Result: {}", part_2(&input));
}

fn parse(s: &str) -> u64 {
    s[4..s.len() - 1]
        .split_once(",")
        .map(|(a, b)| a.parse::<u64>().unwrap() * b.parse::<u64>().unwrap())
        .unwrap()
}

fn part_1(s: &str) -> u64 {
    let mul_pattern = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    mul_pattern.find_iter(s).map(|m| parse(m.as_str())).sum()
}

fn part_2(s: &str) -> u64 {
    let mut total = 0;
    let mut enabled = true;

    let con_mul_pattern = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)|don't\(\)|do\(\)").unwrap();

    for mat in con_mul_pattern.find_iter(s) {
        match mat.as_str() {
            "don't()" => enabled = false,
            "do()" => enabled = true,
            _ => {
                if enabled {
                    total += parse(mat.as_str());
                }
            }
        }
    }

    total
}
