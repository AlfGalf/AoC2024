use std::fmt::Formatter;

fn main() {
    let input = include_str!("../../inputs/day9.txt");

    let input: Vec<usize> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    #[derive(Copy, Clone)]
    enum Blocks {
        None,
        Block(usize),
    }

    impl std::fmt::Debug for Blocks {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Blocks::None => {
                    write!(f, ".")
                }
                Blocks::Block(i) => {
                    write!(f, "{}", i)
                }
            }
        }
    }

    let mut arr: Vec<Blocks> = input
        .iter()
        .fold((0, true, vec![]), |(mut id, block, mut v), x| {
            if block {
                v.push(vec![Blocks::Block(id); *x]);
                id += 1;
            } else {
                v.push(vec![Blocks::None; *x]);
            }
            (id, !block, v)
        })
        .2
        .into_iter()
        .flatten()
        .collect();

    let mut empty_p = 0;
    let mut remove_p = arr.len() - 1;

    fn dbg_arr(arr: &Vec<Blocks>) {
        print!("[");
        for x in arr {
            print!("{:?} ", x);
        }
        println!("]");
    }

    while empty_p < remove_p && empty_p < arr.len() && remove_p > 0 {
        if let &Blocks::Block(_) = &arr[empty_p] {
            empty_p += 1;
            continue;
        }
        if let &Blocks::None = &arr[remove_p] {
            remove_p -= 1;
            continue;
        }
        arr[empty_p] = arr[remove_p];
        arr[remove_p] = Blocks::None;
        // dbg_arr(&arr);
    }

    let res = arr
        .iter()
        .enumerate()
        .filter_map(|(i, b)| match b {
            &Blocks::Block(n) => Some(n * i),
            _ => None,
        })
        .sum::<usize>();

    println!("P1: {}", res);

    let mut arr: Vec<(Blocks, usize)> = input
        .iter()
        .fold((0, true, vec![]), |(mut id, block, mut v), x| {
            if block {
                v.push((Blocks::Block(id), *x));
                id += 1;
            } else {
                v.push((Blocks::None, *x));
            }
            (id, !block, v)
        })
        .2;

    fn dbg_arr2(arr: &Vec<(Blocks, usize)>) {
        print!("[");
        for (x, n) in arr {
            for _ in 0..*n {
                print!("{:?}", x);
            }
        }
        println!("]");
    }

    fn combine_gaps(arr: &mut Vec<(Blocks, usize)>) {
        let mut i = 0;
        while i + 1 < arr.len() {
            while let (Some((Blocks::None, s1)), Some((Blocks::None, s2))) =
                (arr.get(i), arr.get(i + 1))
            {
                arr[i] = (Blocks::None, s1 + s2);
                arr.remove(i + 1);
            }
            i += 1;
        }
    }

    // dbg_arr2(&arr);

    let top_id = arr.iter().fold(0, |n, b| match b {
        (Blocks::Block(i), _) => *i.max(&n),
        _ => n,
    });

    'main: for id in (0..=top_id).rev() {
        let (pos, &(_, size)) = arr
            .iter()
            .enumerate()
            .find(|(_, (b, _))| {
                if let Blocks::Block(n) = b {
                    *n == id
                } else {
                    false
                }
            })
            .unwrap();

        // dbg!(&pos);
        // dbg!(&size);

        for (i, b) in arr[0..pos].iter().enumerate() {
            if let &(Blocks::None, empty_size) = b {
                if empty_size >= size {
                    if empty_size == size {
                        arr[i] = arr[pos];
                        arr[pos] = (Blocks::None, size.clone());
                    } else {
                        arr[i] = arr[pos];
                        arr[pos] = (Blocks::None, size.clone());
                        arr.insert(i + 1, (Blocks::None, empty_size - size));
                    }

                    combine_gaps(&mut arr);

                    // dbg_arr2(&arr);
                    continue 'main;
                }
            }
        }
    }

    let arr = arr.into_iter().flat_map(|(b, s)| vec![b; s]);
    let res = arr
        .enumerate()
        .filter_map(|(i, b)| match b {
            Blocks::Block(n) => Some(n * i),
            _ => None,
        })
        .sum::<usize>();

    println!("P2: {}", res);
}
