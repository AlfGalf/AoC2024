fn main() {
    let input = include_str!("../../inputs/day4.txt");

    let input: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let dirs: [[(isize, isize); 3]; 4] = [
        [(1, 0), (2, 0), (3, 0)],
        [(1, 1), (2, 2), (3, 3)],
        [(0, 1), (0, 2), (0, 3)],
        [(-1, 1), (-2, 2), (-3, 3)],
    ];

    let mut num = 0;
    let width = input.len() as isize;
    let height = input[0].len() as isize;
    for x in 0..width {
        for y in 0..height {
            if input[x as usize][y as usize] != 'X' {
                continue;
            }
            'direction: for dir in &dirs {
                for ((dx, dy), c) in dir.iter().zip("MAS".chars()) {
                    if !(x + *dx >= 0 && x + *dx < width && y + *dy >= 0 && y + *dy < height) {
                        continue 'direction;
                    }
                    if input[(x + *dx) as usize][(y + *dy) as usize] != c {
                        continue 'direction;
                    }
                }
                num += 1;
            }
            'direction: for dir in &dirs {
                for ((dx, dy), c) in dir.iter().zip("MAS".chars()) {
                    if !(x - *dx >= 0 && x - *dx < width && y - *dy >= 0 && y - *dy < height) {
                        continue 'direction;
                    }
                    if input[(x - *dx) as usize][(y - *dy) as usize] != c {
                        continue 'direction;
                    }
                }
                num += 1;
            }
        }
    }

    println!("P1: {}", num);

    let pos: [(isize, isize, char); 5] = [
        (-1, 1, 'M'),
        (-1, -1, 'M'),
        (0, 0, 'A'),
        (1, 1, 'S'),
        (1, -1, 'S'),
    ];

    let check = |p: (isize, isize), d: u8| -> bool {
        for (dx, dy, c) in pos {
            let (dx, dy) = match d {
                0 => (dx, dy),
                1 => (dy, -dx),
                2 => (-dx, -dy),
                3 => (-dy, dx),
                _ => panic!(),
            };

            let (nx, ny) = (p.0 + dx, p.1 + dy);

            if nx < 0 || nx >= width || ny < 0 || ny >= height { return false }

            if input[nx as usize][ny as usize] != c { return false }
        }
        true
    };

    let mut num = 0;
    for x in 0..width {
        for y in 0..height {
            if check((x, y), 0) {
                num += 1;
            }
            if check((x, y), 1) {
                num += 1;
            }
            if check((x, y), 2) {
                num += 1;
            }
            if check((x, y), 3) {
                num += 1;
            }
        }
    }

    println!("P2: {}", num);
}
