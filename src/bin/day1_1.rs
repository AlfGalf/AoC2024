fn main() {
    let input = include_str!("../../inputs/day1_1.txt");

    let pairs: Vec<(usize, usize)> = input.lines().map(|l| {
        let mut nums = l.split(" ").filter(|s| !s.is_empty());
        (nums.next().unwrap().parse().unwrap(), nums.next().unwrap().parse().unwrap())
    }).collect();

    let mut arrays: (Vec<usize>, Vec<usize>) = pairs.into_iter().unzip();

    arrays.0.sort_unstable();
    arrays.1.sort_unstable();

    println!("{}", arrays.0.iter().zip(arrays.1).map(|(a, b)| a.abs_diff(b)).sum::<usize>());
}