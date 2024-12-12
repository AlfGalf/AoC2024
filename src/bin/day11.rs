use std::collections::HashMap;
use num::BigInt;

fn main() {
    let input = include_str!("../../inputs/day11.txt");

    let vals: Vec<BigInt> = input.split(" ").map(|s| s.parse::<BigInt>().unwrap()).collect();

    let mut map: HashMap<(BigInt, usize), usize> = HashMap::new();

    fn iter(map: &mut HashMap<(BigInt, usize), usize>, num: BigInt, i: usize) -> usize {
        if let Some(n) = map.get(&(num.clone(), i)) {
            return *n;
        }

        if i == 0 {
            return 1;
        }

        let res = if num == BigInt::from(0) {
            iter(map, BigInt::from(1), i - 1)
        } else if (num.clone().to_string().len() % 2 == 0) {
            let str = num.clone().to_string();
            let n1 = str[..(str.len()/2)].parse::<BigInt>().unwrap();
            let n2 = str[str.len()/2..].parse::<BigInt>().unwrap();
            iter(map, n1, i - 1) + iter(map, n2, i - 1)
        } else {
            iter(map, num.clone() * 2024, i - 1)
        };
        map.insert((num, i), res);
        res
    }

    let res = vals.clone().into_iter().map(|n| iter(&mut map, n, 25)).sum::<usize>();
    println!("P1: {}", res);

    let res = vals.into_iter().map(|n| iter(&mut map, n, 75)).sum::<usize>();
    println!("P2: {}", res);
}
