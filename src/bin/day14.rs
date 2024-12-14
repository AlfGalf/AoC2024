use num::traits::Euclid;
use regex::Regex;

const INPUT: &'static str = include_str!("../../inputs/day14.txt");
const WIDTH: usize = 101;
const HEIGHT: usize = 103;

// const INPUT: &'static str = include_str!("../../inputs/day14test.txt");
// const WIDTH: usize = 11;
// const HEIGHT: usize = 7;

fn main() {
    let regex = Regex::new(r"(?m)^p=(\d+),(\d+) v=(-?\d+),(-?\d+)$").unwrap();

    let input = regex.captures_iter(INPUT);

    type Coord = (usize, usize);
    type Motion = (usize, usize);

    let input: Vec<(Coord, Motion)> = input
        .map(|c| {
            (
                (
                    dbg!(c.get(1).unwrap().as_str()).parse().unwrap(),
                    c.get(2).unwrap().as_str().parse().unwrap(),
                ),
                (
                    c.get(3)
                        .unwrap()
                        .as_str()
                        .parse::<isize>()
                        .unwrap()
                        .rem_euclid(WIDTH as isize) as usize,
                    c.get(4)
                        .unwrap()
                        .as_str()
                        .parse::<isize>()
                        .unwrap()
                        .rem_euclid(HEIGHT as isize) as usize,
                ),
            )
        })
        .collect();

    // println!("input: {:?}", input);
    fn get_pos((start, mov): (Coord, Motion), t: usize) -> Coord {
        (
            (start.0 + (mov.0 * (t % WIDTH))) % WIDTH,
            (start.1 + mov.1 * (t % HEIGHT)) % HEIGHT,
        )
    }

    fn get_positions(start: &Vec<(Coord, Motion)>, t: usize) -> Vec<Coord> {
        start.iter().map(|&m| get_pos(m, t)).collect()
    }

    fn print_map(sits: &Vec<(Coord, Motion)>, t: usize) {
        let poss = get_positions(sits, t);
        println!("t: {t}");
        for x in 0..HEIGHT {
            for y in 0..WIDTH {
                print!("{}", poss.iter().filter(|p| **p == (y, x)).count())
            }
            println!();
        }
    }

    print_map(&input, 100);

    let quads: (usize, usize, usize, usize) =
        input.iter().fold((0, 0, 0, 0), |(a1, a2, a3, a4), sit| {
            let pos = get_pos(*sit, 100);
            if pos.0 < (WIDTH) / 2 && pos.1 < (HEIGHT) / 2 {
                (a1 + 1, a2, a3, a4)
            } else if pos.0 > (WIDTH) / 2 && pos.1 < (HEIGHT) / 2 {
                (a1, a2 + 1, a3, a4)
            } else if pos.0 < (WIDTH) / 2 && pos.1 > (HEIGHT) / 2 {
                (a1, a2, a3 + 1, a4)
            } else if pos.0 > (WIDTH) / 2 && pos.1 > (HEIGHT) / 2 {
                (a1, a2, a3, a4 + 1)
            } else {
                (a1, a2, a3, a4)
            }
        });
    println!("{:?}", quads);

    let res = quads.0 * quads.1 * quads.2 * quads.3;

    println!("P1: {}", res);

    fn get_score(pos: Vec<Coord>) -> f64 {
        let num = pos.len();
        let n = pos
            .iter()
            .filter(|&&p| {
                p.0 > WIDTH / 3 && p.0 < WIDTH * 2 / 3 && p.1 > HEIGHT / 3 && p.1 < HEIGHT * 2 / 3
            })
            .count();
        n as f64 / num as f64
    }

    let mut best = 0.0;
    let mut best_t = 0;
    for t in 0..10000 {
        let pos = get_positions(&input, t);
        let score = get_score(pos);
        if score > best {
            best = score;
            best_t = t;
            print_map(&input, t);
        }
    }

    println!("P2: {}", best_t);
}
