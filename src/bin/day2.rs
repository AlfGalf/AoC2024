fn main() {
    let input = include_str!("../../inputs/day2.txt");

    let pairs = input
        .lines()
        .map(|l| {
            let nums = l.split(" ").filter(|s| !s.is_empty());
            nums.into_iter()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    fn check(v: &Vec<usize>) -> bool {
        v.iter()
            .zip(v.iter().skip(1))
            .all(|(a, b)| a > b && a - b <= 3)
            || v.iter()
                .zip(v.iter().skip(1))
                .all(|(a, b)| b > a && b - a <= 3)
    }

    let res = pairs.iter().filter(|l| check(&l)).count();

    println!("P1: {res}");

    let res2 = &pairs
        .iter()
        .filter(|l| {
            if check(&l) {
                return true;
            }
            (0..l.len()).any(|i| {
                check(
                    &l.iter()
                        .enumerate()
                        .filter_map(|(ind, v)| if ind != i { Some(*v) } else { None })
                        .collect(),
                )
            })
        })
        .count();

    println!("P2: {res2}");
}
