use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../../inputs/day8.txt");

    type Coord = (usize, usize);

    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let nodes: Vec<(Coord, char)> = input
        .lines()
        .into_iter()
        .enumerate()
        .flat_map(|(x, l)| {
            l.chars()
                .enumerate()
                .filter(|(i, c)| *c != '.')
                .map(move |(y, c)| ((x, y), c))
        })
        .collect();

    let node_map: HashMap<char, Vec<(usize, usize)>> =
        nodes.iter().fold(HashMap::new(), |mut acc, ((x, y), c)| {
            acc.entry(*c)
                .and_modify(|v| v.push((*x, *y)))
                .or_insert(vec![(*x, *y)]);
            acc
        });

    let mut antinodes: HashSet<Coord> = Default::default();

    let get_antinode = |p1: Coord, p2: Coord| -> Option<Coord> {
        let diff: (isize, isize) = (p1.0 as isize - p2.0 as isize, p1.1 as isize - p2.1 as isize);

        let r = (p1.0 as isize + diff.0, p1.1 as isize + diff.1);
        let r = if r.0 >= 0 && r.0 < height as isize && r.1 >= 0 && r.1 < width as isize {
            Some((r.0 as usize, r.1 as usize))
        } else {
            None
        };

        r
    };

    for (_, v) in &node_map {
        // println!("{:?}, {:?}", c, v);
        for p in v.iter().permutations(2) {
            let v1 = p[0];
            let v2 = p[1];

            if let Some(an) = get_antinode(*v1, *v2) {
                antinodes.insert(an);
            }
            if let Some(an) = get_antinode(*v2, *v1) {
                antinodes.insert(an);
            }
        }
    }

    println!("P1: {}", antinodes.len());

    let mut antinodes: HashSet<Coord> = Default::default();

    let get_harmonic_antinodes = |p1: Coord, p2: Coord| -> Vec<Coord> {
        let diff: (isize, isize) = (p1.0 as isize - p2.0 as isize, p1.1 as isize - p2.1 as isize);

        let mut res = vec![];
        for i in (0..) {
            let r = (p1.0 as isize + diff.0 * i, p1.1 as isize + diff.1 * i);
            if r.0 >= 0 && r.0 < height as isize && r.1 >= 0 && r.1 < width as isize {
                res.push((r.0 as usize, r.1 as usize));
            } else {
                break;
            }
        }
        res
    };

    for (_, v) in &node_map {
        // println!("{:?}, {:?}", c, v);
        for p in v.iter().permutations(2) {
            let v1 = p[0];
            let v2 = p[1];

            for an in get_harmonic_antinodes(*v1, *v2) {
                antinodes.insert(an);
            }
            for an in get_harmonic_antinodes(*v2, *v1) {
                antinodes.insert(an);
            }
        }
    }

    for x in 0..width {
        for y in 0..height {
            if antinodes.contains(&(x, y)) {
                print!("#");
            } else {
                print!("{}", input.lines().nth(x).unwrap().chars().nth(y).unwrap());
            }
        }
        println!();
    }

    println!("P2: {}", antinodes.len());
}
