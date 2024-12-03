pub fn run() {
    let input = std::fs::read_to_string("input/day02.txt").unwrap();

    let parsed: Vec<Vec<u64>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();

    println!("Part1 Result: {}", part_1(&parsed));
    println!("Part2 Result: {}", part_2(&parsed));
}

fn is_report_safe(report: &[u64]) -> bool {
    let mut ascending = true;
    let mut descending = true;

    for i in 1..report.len() {
        let cur = report[i];
        let last = report[i - 1];

        let diff = cur.abs_diff(last);
        if !(1..=3).contains(&diff) {
            return false;
        }

        if cur > last {
            descending = false;
        } else {
            ascending = false;
        }

        if !ascending && !descending {
            return false;
        }
    }

    true
}

fn part_1(reports: &[Vec<u64>]) -> u64 {
    reports
        .iter()
        .filter(|report| is_report_safe(report))
        .count() as u64
}

fn part_2(reports: &[Vec<u64>]) -> u64 {
    reports
        .iter()
        .filter(|report| {
            if is_report_safe(report) {
                return true;
            }

            for i in 0..report.len() {
                let mut temp = report.to_vec();
                temp.remove(i);

                if is_report_safe(&temp) {
                    return true;
                }
            }

            false
        })
        .count() as u64
}
