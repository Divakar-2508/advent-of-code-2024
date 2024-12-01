pub fn run() {
    let input = std::fs::read_to_string("input/day01.txt").unwrap();

    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in input.lines() {
        let eles: Vec<u64> = line
            .split_whitespace()
            .map(|ele| ele.parse::<u64>().unwrap())
            .collect();

        list1.push(eles[0]);
        list2.push(eles[1]);
    }

    println!(
        "Total distance between lists: {}",
        total_distance_between_lists(&mut list1, &mut list2)
    );

    println!(
        "Similarity between lists: {}",
        similarity_between_lists(&list1, &list2)
    );
}

fn total_distance_between_lists(list1: &mut [u64], list2: &mut [u64]) -> u64 {
    list1.sort_unstable();
    list2.sort_unstable();

    list1
        .iter()
        .zip(list2.iter())
        .map(|(x, y)| x.abs_diff(*y))
        .sum()
}

/*
assuming lists are sorted by previous challenge
We gotta check if it's sorted first and sort it,
i just think this is clean as of now and based on code structure.
*/
fn similarity_between_lists(list1: &[u64], list2: &[u64]) -> u64 {
    let mut total = 0;
    let mut i = 0;
    let mut j = 0;

    while i < list1.len() {
        let current = list1[i];
        let mut count = 0;

        while j < list2.len() && list2[j] <= current {
            if list2[j] == current {
                count += 1;
            }

            j += 1;
        }

        let current_total = current * count;
        total += current_total;
        i += 1;

        while i < list1.len() && list1[i] == current {
            total += current_total;
        }
    }

    total
}
