use regex::Regex;

fn main() {
    let input = include_str!("../../inputs/day3.txt");

    enum Command {
        Do,
        Dont,
        Mul(usize, usize),
    }

    let muls: Vec<Command> =
        Regex::new(r"(?<mul>mul\((?<a>\d+),(?<b>\d+)\))|(?<do>do\(\))|(?<dont>don't\(\))")
            .unwrap()
            .captures_iter(input)
            .map(|c| {
                if let Some(_) = c.name("mul") {
                    return Command::Mul(
                        c.name("a").unwrap().as_str().parse().unwrap(),
                        c.name("b").unwrap().as_str().parse().unwrap(),
                    );
                }
                if let Some(_) = c.name("do") {
                    return Command::Do;
                }
                if let Some(_) = c.name("dont") {
                    return Command::Dont;
                }
                panic!("Unknown command");
            })
            .collect();

    let res1 = muls
        .iter()
        .filter_map(|c| match c {
            Command::Mul(a, b) => Some(a * b),
            _ => None,
        })
        .sum::<usize>();
    println!("P1: {}", res1);

    let res2 = muls
        .iter()
        .fold((true, 0), |(is_on, acc), c| match c {
            Command::Do => (true, acc),
            Command::Dont => (false, acc),
            Command::Mul(a, b) => (is_on, if is_on { acc + a * b } else { acc }),
        })
        .1;
    println!("P2: {}", res2);
}
