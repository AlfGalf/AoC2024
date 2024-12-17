use std::cmp::PartialEq;
use std::fmt::{write, Debug};

const INPUT: &'static str = include_str!("../../inputs/day15.txt");
// const INPUT: &'static str = include_str!("../../inputs/day15test.txt");
// const INPUT: &'static str = include_str!("../../inputs/day15test2.txt");

fn main() {
    #[derive(PartialEq, Eq, Copy, Clone)]
    enum Tile {
        Empty,
        Box,
        Wall,
        BoxLeft,
        BoxRight,
    }

    impl Debug for Tile {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Tile::Empty => " ",
                    Tile::Box => "O",
                    Tile::Wall => "#",
                    Tile::BoxLeft => "[",
                    Tile::BoxRight => "]",
                }
            )
        }
    }

    let mut pos = (0, 0);

    let mut input = INPUT.lines();

    let mut map: Vec<Vec<Tile>> = (&mut input)
        .take_while(|s| s != &"")
        .enumerate()
        .map(|(x, l)| {
            l.chars()
                .enumerate()
                .map(|(y, c)| match c {
                    '@' => {
                        pos = (x, y);
                        Tile::Empty
                    }
                    '.' => Tile::Empty,
                    '#' => Tile::Wall,
                    'O' => Tile::Box,
                    _ => panic!(),
                })
                .collect()
        })
        .collect();

    let mut p2_pos = (pos.0, pos.1 * 2);

    let width = map.len();
    let height = map[0].len();

    #[derive(Debug, Copy, Clone)]
    enum Commands {
        Up,
        Down,
        Left,
        Right,
    }
    let commands: Vec<Commands> = input
        .flat_map(|l| {
            l.chars().map(|c| match c {
                '>' => Commands::Right,
                '<' => Commands::Left,
                '^' => Commands::Up,
                'v' => Commands::Down,
                _ => panic!(),
            })
        })
        .collect();
    impl Commands {
        fn get_dir(&self) -> (isize, isize) {
            match self {
                Commands::Up => (-1, 0),
                Commands::Down => (1, 0),
                Commands::Left => (0, -1),
                Commands::Right => (0, 1),
            }
        }
    }

    // dbg!(pos);
    // dbg!(map);
    // dbg!(commands);

    fn make_move(
        map: &mut Vec<Vec<Tile>>,
        (x, y): (usize, usize),
        command: Commands,
    ) -> (usize, usize) {
        let dir: (isize, isize) = command.get_dir();

        let to_pos = (
            x.checked_add_signed(dir.0).unwrap(),
            y.checked_add_signed(dir.1).unwrap(),
        );
        if map[to_pos.0][to_pos.1] == Tile::Empty {
            return to_pos;
        }
        let mut box_pos = to_pos.clone();

        loop {
            match map[box_pos.0][box_pos.1] {
                Tile::Empty => {
                    map[box_pos.0][box_pos.1] = Tile::Box;
                    map[to_pos.0][to_pos.1] = Tile::Empty;
                    break to_pos;
                }
                Tile::Box => {
                    box_pos = (
                        box_pos.0.saturating_add_signed(dir.0),
                        box_pos.1.saturating_add_signed(dir.1),
                    )
                }
                Tile::Wall => break (x, y),
                _ => panic!(),
            }
        }
    }

    fn print_map(map: &Vec<Vec<Tile>>, pos: (usize, usize), (width, height): (usize, usize)) {
        for x in 0..width {
            for y in 0..height {
                if (x, y) == pos {
                    print!("@");
                    continue;
                }
                print!("{:?}", map[x][y]);
            }
            println!();
        }
    }

    // print_map(&map, pos, (width, height));
    for &command in &commands {
        // println!("{:?}", command);
        pos = make_move(&mut map, pos, command);
        // print_map(&map, pos, (width, height));
    }

    let res = map
        .iter()
        .enumerate()
        .flat_map(move |(y, row)| {
            row.iter().enumerate().filter_map(move |(x, tile)| {
                if let Tile::Box = tile {
                    Some(y * 100 + x)
                } else {
                    None
                }
            })
        })
        .sum::<usize>();

    println!("P1: {}", res);

    let mut map: Vec<Vec<Tile>> = INPUT
        .lines()
        .take_while(|s| s != &"")
        .map(|l| {
            l.chars()
                .flat_map(|c| match c {
                    '@' => [Tile::Empty, Tile::Empty],
                    '.' => [Tile::Empty, Tile::Empty],
                    '#' => [Tile::Wall, Tile::Wall],
                    'O' => [Tile::BoxLeft, Tile::BoxRight],
                    _ => panic!(),
                })
                .collect()
        })
        .collect();

    fn move_p2(map: &mut Vec<Vec<Tile>>, (x, y): (usize, usize), dir: (isize, isize)) -> bool {
        match map[x][y] {
            Tile::Empty => true,
            Tile::Wall => false,
            Tile::BoxLeft | Tile::BoxRight if dir.0 == 0 => {
                if move_p2(map, (x, y.saturating_add_signed(dir.1)), dir) {
                    map[x][y.saturating_add_signed(dir.1)] = map[x][y];
                    map[x][y] = Tile::Empty;
                    true
                } else {
                    false
                }
            }
            Tile::BoxLeft => {
                if move_p2(map, (x.saturating_add_signed(dir.0), y), dir)
                    && move_p2(map, (x.saturating_add_signed(dir.0), y + 1), dir)
                {
                    map[x.saturating_add_signed(dir.0)][y] = Tile::BoxLeft;
                    map[x.saturating_add_signed(dir.0)][y + 1] = Tile::BoxRight;
                    map[x][y] = Tile::Empty;
                    map[x][y + 1] = Tile::Empty;
                    true
                } else {
                    false
                }
            }
            Tile::BoxRight => {
                if move_p2(map, (x.saturating_add_signed(dir.0), y), dir)
                    && move_p2(map, (x.saturating_add_signed(dir.0), y - 1), dir)
                {
                    map[x.saturating_add_signed(dir.0)][y] = Tile::BoxRight;
                    map[x.saturating_add_signed(dir.0)][y - 1] = Tile::BoxLeft;
                    map[x][y] = Tile::Empty;
                    map[x][y - 1] = Tile::Empty;
                    true
                } else {
                    false
                }
            }
            Tile::Box => panic!(),
        }
    }

    // print_map(&map, (p2_pos.0, p2_pos.1), (width, height * 2));
    for &command in &commands {
        // println!("{:?}", command);
        let map_backup = map.clone();
        let dir: (isize, isize) = command.get_dir();
        if move_p2(
            &mut map,
            (
                p2_pos.0.saturating_add_signed(dir.0),
                p2_pos.1.saturating_add_signed(dir.1),
            ),
            dir,
        ) {
            p2_pos = (
                p2_pos.0.saturating_add_signed(dir.0),
                p2_pos.1.saturating_add_signed(dir.1),
            );
        } else {
            map = map_backup;
        }
        // print_map(&map, (p2_pos.0, p2_pos.1), (width, height * 2));
    }

    let res = map
        .iter()
        .enumerate()
        .flat_map(move |(y, row)| {
            row.iter().enumerate().filter_map(move |(x, tile)| {
                if let Tile::BoxLeft = tile {
                    Some(y * 100 + x)
                } else {
                    None
                }
            })
        })
        .sum::<usize>();

    println!("P2: {}", res);
}
