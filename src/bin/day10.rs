use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("../../inputs/day10.txt");

    let map: Vec<Vec<Option<usize>>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).map(|c| c as usize))
                .collect()
        })
        .collect();

    let width = map.len();
    let height = map[0].len();

    fn explore(
        x: usize,
        y: usize,
        map: &Vec<Vec<Option<usize>>>,
        width: usize,
        height: usize,
    ) -> usize {
        let mut to_explore = VecDeque::from([(x, y)]);
        let mut explored = HashSet::new();
        let mut num_9 = 0;

        while let Some((x, y)) = to_explore.pop_front() {
            if explored.contains(&(x, y)) {
                continue;
            }
            explored.insert((x, y));
            let Some(node) = map[x][y] else { continue };
            if node == 9 {
                num_9 += 1;
                continue;
            }
            if x > 0 && map[x - 1][y] == Some(node + 1) {
                to_explore.push_back((x - 1, y));
            }
            if y > 0 && map[x][y - 1] == Some(node + 1) {
                to_explore.push_back((x, y - 1));
            }
            if x + 1 < width && map[x + 1][y] == Some(node + 1) {
                to_explore.push_back((x + 1, y));
            }
            if y + 1 < height && map[x][y + 1] == Some(node + 1) {
                to_explore.push_back((x, y + 1));
            }
        }
        num_9
    }

    let mut res = 0;
    for x in 0..width {
        for y in 0..height {
            if map[x][y] == Some(0) {
                res += explore(x, y, &map, width, height);
            }
        }
    }
    println!("P1: {}", res);

    fn explore_rec(
        x: usize,
        y: usize,
        map: &Vec<Vec<Option<usize>>>,
        width: usize,
        height: usize,
    ) -> usize {
        let Some(node) = map[x][y] else {
            return 0;
        };
        if node == 9 {
            return 1;
        }
        let mut res = 0;
        if x > 0 && map[x - 1][y] == Some(node + 1) {
            res += explore_rec(x - 1, y, map, width, height);
        }
        if y > 0 && map[x][y - 1] == Some(node + 1) {
            res += explore_rec(x, y - 1, map, width, height);
        }
        if x + 1 < width && map[x + 1][y] == Some(node + 1) {
            res += explore_rec(x + 1, y, map, width, height);
        }
        if y + 1 < height && map[x][y + 1] == Some(node + 1) {
            res += explore_rec(x, y + 1, map, width, height);
        }
        res
    }

    let mut res = 0;
    for x in 0..width {
        for y in 0..height {
            if map[x][y] == Some(0) {
                res += explore_rec(x, y, &map, width, height);
            }
        }
    }
    println!("P2: {}", res);
}
