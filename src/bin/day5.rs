use regex::Regex;
use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    let input = include_str!("../../inputs/day5.txt");

    let rules: HashMap<usize, Vec<usize>> = Regex::new(r"(\d+)\|(\d+)")
        .unwrap()
        .captures_iter(input)
        .fold(HashMap::new(), |mut hm, c| {
            let v: usize = c.get(1).unwrap().as_str().parse().unwrap();
            let r: usize = c.get(2).unwrap().as_str().parse().unwrap();
            hm.entry(v).and_modify(|vec| vec.push(r)).or_insert(vec![r]);
            hm
        });

    let mut sequences: Vec<Vec<usize>> = Regex::new(r"(?m)^(\d+)(,\d+)+$")
        .unwrap()
        .captures_iter(input)
        .map(|c| {
            c.get(0)
                .unwrap()
                .as_str()
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();

    let is_sorted = |seq: &Vec<usize>| -> bool {
        for (i, n) in seq.iter().enumerate() {

            if let Some(m) = rules.get(&n) {
                for m in m {
                    if let Some(p) = seq.iter().position(|x| x == m) {
                        if p < i {
                            return false;
                        }
                    }
                }
            }
        }
        true
    };
    let res: usize = sequences
        .iter()
        .filter(|seq| is_sorted(*seq))
        .map(|seq| seq.get((seq.len() - 1) / 2).unwrap())
        .sum();

    println!("P1: {}", res);

    let res: usize = sequences
        .iter_mut()
        .filter(|seq| !is_sorted(seq))
        .map(|mut v| {
            v.sort_by(|a, b| {
                if let Some(ords) = rules.get(a) {
                    if ords.contains(b) {
                        return std::cmp::Ordering::Less;
                    }
                }
                if let Some(ords) = rules.get(b) {
                    if ords.contains(a) {
                        return std::cmp::Ordering::Greater;
                    }
                }
                std::cmp::Ordering::Equal
            });
            v
        })
        .map(|seq| seq.get((seq.len() - 1) / 2).unwrap())
        .sum();

    println!("P2: {}", res);
}
