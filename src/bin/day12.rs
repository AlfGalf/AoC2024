use ndarray::{indices, Array2};
use std::collections::VecDeque;

fn main() {
    let input = include_str!("../../inputs/day12.txt");

    let chars: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut region_chars: Vec<(char, (usize, usize), usize, usize, usize)> = vec![];
    let mut regions: Array2<Option<usize>> = Array2::from_elem((chars.len(), chars[0].len()), None);

    type Coord = (isize, isize);
    let sides: [Coord; 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let sides_inherit: [Coord; 4] = [(0, -1), (0, -1), (-1, 0), (-1, 0)];

    for x in indices(regions.shape()) {
        let y: usize = x[1];
        let x: usize = x[0];
        let ob = regions[[x, y]];
        if ob.is_some() {
            continue;
        }
        let char = chars[x][y];

        let n = region_chars.len();
        let mut region_char = (char, (x, y), 0, 0, 0);

        let is_edge = |(x, y): Coord, (dx, dy): Coord| -> bool {
            let (x1, y1) = (x + dx, y + dy);
            if x1 < 0 || x1 >= chars.len() as isize || y1 < 0 || y1 >= chars[0].len() as isize {
                return true;
            }
            chars[x1 as usize][y1 as usize] != chars[x as usize][y as usize]
        };

        let mut frontier = VecDeque::from(vec![(x, y)]);
        while let Some((x, y)) = frontier.pop_front() {
            if regions[[x, y]].is_some() {
                continue;
            }
            if chars[x][y] != char {
                continue;
            }

            // Area
            region_char.2 += 1;
            // Mark region
            regions[[x, y]] = Some(n);

            let (x, y) = (x as isize, y as isize);
            for ((dx, dy), (sdx, sdy)) in sides.into_iter().zip(sides_inherit.into_iter()) {
                let (x1, y1) = (x + dx, y + dy);

                if is_edge((x, y), (dx, dy)) {
                    region_char.3 += 1;

                    if is_edge((x, y), (sdx, sdy)) // ie, the inherit side is still in the area
                        || !is_edge((x + sdx, y + sdy), (dx, dy))
                    {
                        // ie, there is no edge to inherit from
                        region_char.4 += 1;
                    }
                    continue;
                }

                if regions[(x1 as usize, y1 as usize)] == None
                    && chars[x1 as usize][y1 as usize] == char
                {
                    frontier.push_back((x1 as usize, y1 as usize));
                }
            }
        }
        region_chars.push(region_char);
    }

    // let regions = regions.map(|c| c.unwrap());
    // println!("{:?}", regions);
    // println!("{:?}", region_chars);
    let res = region_chars
        .iter()
        .map(|(_, _, a, p, _)| a * p)
        .sum::<usize>();
    println!("P1: {}", res);
    let res = region_chars
        .iter()
        .map(|(_, _, a, _, e)| a * e)
        .sum::<usize>();
    println!("P2: {}", res);
}
