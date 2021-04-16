use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/16.txt");

    let list = input.chars().map(parse_c).to_vec();

    let offset = parse_u(&input[0..7]);

    assert!(2 * offset >= list.len() * 10_000);

    let mut list = list
        .iter()
        .copied()
        .cycle()
        .take(list.len() * 10_000)
        .skip(offset)
        .to_vec();

    let mut new_list = list.clone();
    for _ in 0..100 {
        let mut sum = 0;
        for (new, old) in new_list.iter_mut().zip(list.iter()).rev() {
            sum += *old;
            *new = sum % 10;
        }

        std::mem::swap(&mut list, &mut new_list);
    }
    for v in list.iter().take(8) {
        print!("{}", v);
    }
    println!();
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/16.txt");

    let mut list = input.chars().map(parse_c).to_vec();

    for _ in 0..100 {
        let mut new_list = list.clone();

        for (i, v) in new_list.iter_mut().enumerate() {
            let pattern = std::iter::repeat(0)
                .take(i + 1)
                .chain(std::iter::repeat(1).take(i + 1))
                .chain(std::iter::repeat(0).take(i + 1))
                .chain(std::iter::repeat(-1).take(i + 1))
                .cycle()
                .skip(1);

            let sum: isize = list.iter().zip(pattern).map(|(v, n)| *v as isize * n).sum();
            *v = (sum % 10).abs() as usize;
        }

        list = new_list;
    }
    for v in list.iter().take(8) {
        print!("{}", v);
    }
    println!();
}
