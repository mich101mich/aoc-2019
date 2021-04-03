use crate::utils::*;

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/22.txt");

    use num_bigint::BigInt as Int;

    // let pos = 2020i128;
    let cards = 119_315_717_514_047i128;
    let iterations = 101_741_582_076_661i128;

    // let pos = 6129i128;
    // let cards = 10007i128;
    // let iterations = 1i128;

    let cards = Int::from(cards);
    let iterations = Int::from(iterations);
    // let pos = Int::from(pos);

    let mut plus = Int::from(0);
    let mut mul = Int::from(1);
    for action in input.lines().rev() {
        if action == "deal into new stack" {
            mul *= -1;
            plus *= -1;
            plus -= 1;
        } else if let Ok(n) = scan_fmt!(action, "cut {}", isize) {
            plus += n;
        } else if let Ok(n) = scan_fmt!(action, "deal with increment {}", isize) {
            let f = Int::from(n).modpow(&(&cards - 2), &cards);
            mul *= &f;
            plus *= f;
        } else {
            panic!("Invalid action")
        }
        mul %= &cards;
        plus %= &cards;
    }

    let pow = mul.modpow(&iterations, &cards);

    // pow
    // 	.multiply(num(2020))
    // 	.add(
    // 		plus
    // 			.multiply(pow + cards - 1)
    // 			.multiply((mul - 1).modPow(cards - 2, cards))
    // 	)
    // 	.mod(deckSize);

    let res = &pow * 2020
        + (&plus * (&pow + &cards - 1) * (&mul - Int::from(1)).modpow(&(&cards - 2), &cards));
    let res = res % cards;
    pv!(res);
    println!("{}", res);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/22.txt");
    // let input = ;
    let parsed = input.lines().to_vec();

    let mut cards = (0..10007).collect::<VecDeque<_>>();

    for action in parsed {
        if action == "deal into new stack" {
            cards = cards.into_iter().rev().collect();
        } else if let Ok(n) = scan_fmt!(action, "cut {}", isize) {
            if n >= 0 {
                for _ in 0..n {
                    let e = cards.pop_front().unwrap();
                    cards.push_back(e);
                }
            } else {
                for _ in 0..(-n) {
                    let e = cards.pop_back().unwrap();
                    cards.push_front(e);
                }
            }
        } else if let Ok(n) = scan_fmt!(action, "deal with increment {}", usize) {
            let remaining = cards.clone();
            let mut pos = 0;
            for e in remaining {
                cards[pos] = e;
                pos = (pos + n) % cards.len();
            }
        }
    }

    pv!(cards.iter().enumerate().find(|(_, n)| **n == 2019));
}
