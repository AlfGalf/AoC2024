use glam::{DMat2, DVec2, Vec2};
use num::ToPrimitive;
use regex::Regex;

fn main() {
    let input = include_str!("../../inputs/day13.txt");

    let regex = Regex::new(r"(?m)^Button A: X\+(?<ax>\d+), Y\+(?<ay>\d+)\nButton B: X\+(?<bx>\d+), Y\+(?<by>\d+)\nPrize: X=(?<px>\d+), Y=(?<py>\d+)$").unwrap();

    let input: Vec<(DMat2, DVec2)> = regex
        .captures_iter(input)
        .map(|c| {
            (
                DMat2::from_cols_array(&[
                    c.name("ax").unwrap().as_str().parse().unwrap(),
                    c.name("ay").unwrap().as_str().parse().unwrap(),
                    c.name("bx").unwrap().as_str().parse().unwrap(),
                    c.name("by").unwrap().as_str().parse().unwrap(),
                ]),
                DVec2::new(
                    c.name("px").unwrap().as_str().parse().unwrap(),
                    c.name("py").unwrap().as_str().parse().unwrap(),
                ),
            )
        })
        .collect();

    fn calc(input: Vec<(DMat2, DVec2)>) -> usize {
        /*
         * [ ax, ay ] [ a ] = [ px ]
         * [ bx, by ] [ b ] = [ py ]
         * [ a ] = [ ax, ay ] ^-1 [ px ]
         * [ b ] = [ bx, by ]     [ py ]
         */
        let res = input.into_iter().filter_map(|(m, v)| {
            if m.determinant() == 0.0 {
                return None;
            }
            let mi = m.inverse();

            Some(mi * v)
        });

        let res = res.filter_map(|m| {
            let a = m.x;
            if (a - a.round()).abs() > 0.001 {
                return None;
            }
            let a = a.round().to_usize().unwrap();
            let b = m.y;
            if (b - b.round()).abs() > 0.001 {
                return None;
            }
            let b = b.round().to_usize().unwrap();
            Some((a, b))
        });

        let res: usize = res.map(|(a, b)| 3 * a + b).sum();

        res
    }

    println!("P1: {}", calc(input.clone()));

    println!(
        "P2: {}",
        calc(
            input
                .into_iter()
                .map(|(m, v)| (m, v + DVec2::splat(10000000000000.0)))
                .collect()
        )
    );
}
