use std::collections::HashSet;

fn main() {
    let input = include_str!("../../inputs/day6.txt");

    let mut map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    #[derive(Eq, Hash, PartialEq, Clone)]
    enum Dir {
        Up,
        Down,
        Left,
        Right,
    }
    type State = (usize, usize, Dir);

    let start: State = map
        .iter()
        .enumerate()
        .filter_map(|(x, l)| {
            l.iter()
                .enumerate()
                .filter_map(|(y, c)| {
                    if *c == '^' {
                        Some((x, y, Dir::Up))
                    } else {
                        None
                    }
                })
                .next()
        })
        .next()
        .unwrap();

    fn time_to_exit(start: State, map: &Vec<Vec<char>>) -> Option<usize> {
        let mut history: HashSet<State> = Default::default();
        let mut visited: HashSet<(usize, usize)> = Default::default();

        let mut current = start;

        let next_straight = |state: &State| -> Option<State> {
            match state.2 {
                Dir::Up => {
                    if state.0 > 0 {
                        Some((state.0 - 1, state.1, Dir::Up))
                    } else {
                        None
                    }
                }
                Dir::Down => {
                    if state.0 + 1 < map.len() {
                        Some((state.0 + 1, state.1, Dir::Down))
                    } else {
                        None
                    }
                }
                Dir::Left => {
                    if state.1 > 0 {
                        Some((state.0, state.1 - 1, Dir::Left))
                    } else {
                        None
                    }
                }
                Dir::Right => {
                    if state.1 + 1 < map[0].len() {
                        Some((state.0, state.1 + 1, Dir::Right))
                    } else {
                        None
                    }
                }
            }
        };

        fn next_rotate(state: &State) -> State {
            match state.2 {
                Dir::Up => (state.0, state.1, Dir::Right),
                Dir::Down => (state.0, state.1, Dir::Left),
                Dir::Left => (state.0, state.1, Dir::Up),
                Dir::Right => (state.0, state.1, Dir::Down),
            }
        }

        while !history.contains(&current) {
            history.insert(current.clone());
            visited.insert((current.0, current.1));

            let Some(next_straight) = next_straight(&current) else {
                return Some(visited.len());
            };
            let next_rotate = next_rotate(&current);

            current = if (map[next_straight.0][next_straight.1]) != '#' {
                next_straight
            } else {
                next_rotate
            };
        }
        None
    }

    println!("P1: {}", time_to_exit(start.clone(), &map).unwrap());

    let mut res = 0;
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] != '.' {
                continue;
            }
            map[x][y] = '#';
            if time_to_exit(start.clone(), &map).is_none() {
                res += 1;
            }
            map[x][y] = '.';
        }
    }
    println!("P2: {}", res);
}
