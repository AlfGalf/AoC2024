use std::collections::HashMap;

fn main() {
    let input = include_str!("../../inputs/day1_1.txt");

    let pairs: Vec<(usize, usize)> = input.lines().map(|l| {
        let mut nums = l.split(" ").filter(|s| !s.is_empty());
        (nums.next().unwrap().parse().unwrap(), nums.next().unwrap().parse().unwrap())
    }).collect();

    let arrays: (Vec<usize>, Vec<usize>) = pairs.into_iter().unzip();

    let arrays: (HashMap<usize, usize>, HashMap<usize, usize>) =
        (arrays.0.iter().fold(HashMap::new(), |mut acc, k| {acc.entry(*k).and_modify(|frq| *frq+=1).or_insert(1); acc}),
         arrays.1.iter().fold(HashMap::new(), |mut acc, k| {acc.entry(*k).and_modify(|frq| *frq+=1).or_insert(1); acc}));

    let ans: usize = arrays.0.iter().map(|(k, f)| {
        k * f * arrays.1.get(&k).unwrap_or(&0)
    }).sum();

    println!("{}", ans);
}