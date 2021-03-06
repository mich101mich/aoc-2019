use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/02.txt");
    // let input = "1,9,10,3,2,3,11,0,99,30,40,50";

    let mut parsed = input.split(',').map(parse).to_vec();

    let mut index = 0;

    for a in 0..parsed.len() {
        for b in 0..parsed.len() {
            let mut parsed = parsed.clone();
            index = 0;
            parsed[1] = a as isize;
            parsed[2] = b as isize;

            loop {
                match parsed[index] {
                    1 => {
                        let a = parsed[index + 1] as usize;
                        let b = parsed[index + 2] as usize;
                        let out = parsed[index + 3] as usize;
                        parsed[out] = parsed[a] + parsed[b];
                    }
                    2 => {
                        let a = parsed[index + 1] as usize;
                        let b = parsed[index + 2] as usize;
                        let out = parsed[index + 3] as usize;
                        parsed[out] = parsed[a] * parsed[b];
                    }
                    99 => break,
                    _ => panic!("invalid opcode"),
                }
                index += 4;
            }
            if parsed[0] == 19_690_720 {
                println!("a: {:?}", a);
                println!("b: {:?}", b);
                pv!(100 * a + b);
                return;
            }
        }
    }
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/02.txt");
    // let input = "1,9,10,3,2,3,11,0,99,30,40,50";

    let mut parsed = input.split(',').map(parse).to_vec();

    let mut index = 0;

    parsed[1] = 12;
    parsed[2] = 2;

    loop {
        match parsed[index] {
            1 => {
                let a = parsed[index + 1] as usize;
                let b = parsed[index + 2] as usize;
                let out = parsed[index + 3] as usize;
                parsed[out] = parsed[a] + parsed[b];
            }
            2 => {
                let a = parsed[index + 1] as usize;
                let b = parsed[index + 2] as usize;
                let out = parsed[index + 3] as usize;
                parsed[out] = parsed[a] * parsed[b];
            }
            99 => break,
            _ => panic!("invalid opcode"),
        }
        index += 4;
    }
    pv!(parsed[0]);
}
