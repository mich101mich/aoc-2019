use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/08.txt");

    let parsed = input.chars().to_vec();
    let w = 25;
    let h = 6;

    let mut image = vec!['2'; w * h];

    for layer in parsed.chunks(w * h) {
        for (img, l) in image.iter_mut().zip(layer) {
            if *img == '2' {
                *img = *l;
            }
        }
    }
    for (i, img) in image.iter().enumerate() {
        print!("{}", if *img == '1' { '#' } else { ' ' });
        if i % w == w - 1 {
            println!();
        }
    }
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/08.txt");

    let parsed = input.chars().to_vec();
    let w = 25;
    let h = 6;

    let min_layer = parsed
        .chunks(w * h)
        .min_by_key(|l| l.iter().filter(|c| **c == '0').count())
        .unwrap();

    let ones = min_layer.iter().filter(|c| **c == '1').count();
    let twos = min_layer.iter().filter(|c| **c == '2').count();

    pv!(ones * twos);
}
