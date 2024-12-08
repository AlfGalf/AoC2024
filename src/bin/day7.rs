fn main() {
    let input = include_str!("../../inputs/day7.txt");

    let input: Vec<(usize, Vec<usize>)> = input.lines().map(|s| {
        let mut line = s.split(":");

        let total: usize = line.next().unwrap().parse().unwrap();
        let parts: Vec<usize> = line.next().unwrap().split(" ").filter_map(|x| x.parse().ok()).collect();

        (total, parts)
    }).collect();

    let mut res = 0;
    for (total, parts) in input.clone() {
        fn iter(target: usize, cur: usize, mut next: impl Clone + Iterator<Item = usize>) -> bool {
            if let Some(num) = next.next() {
                if iter(target, cur + num, next.clone()) {true}
                else if iter(target, cur * num, next) {true}
                else {false}
            } else {
                target == cur
            }
        }

        let mut parts = parts.into_iter();
        if iter(total, parts.next().unwrap(), parts) {
            res += total
        }
    }
    println!("P1: {}", res);

    let mut res = 0;
    for (total, parts) in input {
        fn iter(target: usize, cur: usize, mut next: impl Clone + Iterator<Item = usize>) -> bool {
            if let Some(num) = next.next() {
                if cur > target {false}
                else if iter(target, cur + num, next.clone()) {true}
                else if iter(target, cur * num, next.clone()) {true}
                else if iter(target, format!("{cur}{num}").parse().unwrap(), next) {true}
                else {false}
            } else {
                target == cur
            }
        }

        let mut parts = parts.into_iter();
        if iter(total, parts.next().unwrap(), parts) {
            res += total
        }
    }
    println!("P2: {}", res);
}
