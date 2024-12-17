use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Alignment::Right;

const INPUT: &'static str = include_str!("../../inputs/day16.txt");
// const INPUT: &'static str = include_str!("../../inputs/day16test.txt");

const DIRS: [char; 4] = ['<', '>', '^', 'v'];
fn get_dir(c: char) -> (isize, isize) {
    match c {
        '>' => (0, 1),
        '<' => (0, -1),
        '^' => (-1, 0),
        'v' => (1, 0),
        _ => unreachable!(),
    }
}

fn main() {
    #[derive(Eq, PartialEq, Ord)]
    struct State {
        position: (usize, usize),
        dir: char,
        cost: usize,
    }
    impl PartialOrd<Self> for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(
                other
                    .cost
                    .cmp(&self.cost)
                    .then_with(|| self.position.cmp(&other.position))
                    .then_with(|| self.dir.cmp(&other.dir)),
            )
        }
    }

    let mut dists: HashMap<((usize, usize), char), usize> = Default::default();
    let mut queue: BinaryHeap<State> = Default::default();

    let mut start = (0, 0);
    let mut end = (0, 0);
    let map: Vec<Vec<char>> = INPUT
        .lines()
        .enumerate()
        .map(|(x, l)| {
            l.chars()
                .enumerate()
                .map(|(y, c)| match c {
                    'S' => {
                        start = (x, y);
                        '.'
                    }
                    'E' => {
                        end = (x, y);
                        '.'
                    }
                    s => s,
                })
                .collect()
        })
        .collect();

    queue.push(State {
        position: start,
        dir: '>',
        cost: 0,
    });

    let res = loop {
        let State {
            position,
            dir,
            cost,
        } = queue.pop().unwrap();

        // println!("Explore {:?} {} {}", position, dir, cost);
        if dists.get(&(position, dir)).is_some_and(|v| v <= &cost) {
            continue;
        }

        dists.insert((position, dir), cost);

        if position == end {
            break cost;
        }

        let (dir_x, dir_y) = get_dir(dir);

        let (new_x, new_y) = (
            position.0.saturating_add_signed(dir_x),
            position.1.saturating_add_signed(dir_y),
        );
        if map[new_x][new_y] == '.' {
            if dists
                .get(&((new_x, new_y), dir))
                .is_none_or(|v| cost + 1 < *v)
            {
                queue.push(State {
                    position: (new_x, new_y),
                    dir,
                    cost: cost + 1,
                })
            }
        }
        for &ndir in DIRS.iter() {
            if dists
                .get(&(position, ndir))
                .is_none_or(|v| cost + 1000 < *v)
            {
                queue.push(State {
                    position,
                    dir: ndir,
                    cost: cost + 1000,
                })
            }
        }
    };

    println!("P1: {}", res);

    let mut explore: VecDeque<((usize, usize), char, usize)> = Default::default();
    let mut used: HashSet<(usize, usize)> = Default::default();

    explore.push_back((end, '^', res));
    while let Some((pos, dir, cost)) = explore.pop_front() {
        used.insert(pos);
        let (dir_x, dir_y) = get_dir(dir);
        let previous_pos = (
            pos.0.saturating_add_signed(-dir_x),
            pos.1.saturating_add_signed(-dir_y),
        );
        if dists
            .get(&(previous_pos, dir))
            .is_some_and(|c| *c + 1 == cost)
        {
            explore.push_back((previous_pos, dir, cost - 1));
        }

        for dir in DIRS.iter() {
            if dists.get(&(pos, *dir)).is_some_and(|c| *c + 1000 == cost) {
                explore.push_back((pos, *dir, cost - 1000));
            }
        }
    }
    println!("P2: {}", used.len());

    // for x in 0..map.len() {
    //     for y in 0..map[0].len() {
    //         if used.contains(&(x, y)) {
    //             print!("O");
    //         }
    //         else {
    //             print!("{}", map[x][y]);
    //         }
    //     }
    //     println!();
    // }
}
