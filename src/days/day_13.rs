use crate::utils::*;

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/13.txt");

    let mut game = vec![vec![0; 42]; 24];
    let mut code = IntProgram::new(input, vec![]);
    code.mem[0] = 2;

    let mut score = 0;
    let mut ball = -1;
    let mut paddle = 0;
    let mut vel = 0;
    loop {
        if let Some(x) = int_code(&mut code, true) {
            let y = int_code(&mut code, true).unwrap();
            let id = int_code(&mut code, true).unwrap();
            if x < 0 {
                score = id;
            } else {
                if x as usize >= game[0].len() {
                    game.iter_mut().for_each(|r| r.resize(x as usize + 1, 0));
                }
                if y as usize >= game.len() {
                    let w = game[0].len();
                    game.resize_with(y as usize + 1, || vec![0; w]);
                }
                game[y as usize][x as usize] = id;
                if id == 4 {
                    if ball != -1 {
                        vel = x - ball;
                    }
                    ball = x;
                    use std::cmp::Ordering::*;
                    let input = match paddle.cmp(&ball) {
                        Less if vel > 0 => 1,
                        Greater if vel < 0 => -1,
                        _ => 0,
                    };
                    // for _ in 0..paddle - 1 {
                    //     print!(" ");
                    // }
                    // print!(
                    //     "{}-{}\r",
                    //     if input < 0 { '<' } else { ' ' },
                    //     if input > 0 { '>' } else { ' ' }
                    // );
                    // std::io::stdout().flush();
                    // std::thread::sleep_ms(5);

                    paddle += input;
                    code.inputs.push(input);
                } else if id == 3 {
                    paddle = x;
                }
                // if y as usize == game.len() - 1 && x as usize == game[0].len() - 1 {
                //     for row in game.iter() {
                //         for c in row {
                //             print!("{}", [' ', '#', 'x', '-', 'o'][*c as usize]);
                //         }
                //         println!();
                //     }
                //     println!();
                // }
            }
        } else {
            pv!(score);
            return;
        }
    }
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/13.txt");
    let mut game = vec![vec![0; 10]; 10];
    let mut code = IntProgram::new(input, vec![]);
    while let Some(x) = int_code(&mut code, true) {
        let y = int_code(&mut code, true).unwrap();
        let id = int_code(&mut code, true).unwrap();
        if x as usize >= game[0].len() {
            game.iter_mut().for_each(|r| r.resize(x as usize + 1, 0));
        }
        if y as usize >= game.len() {
            let w = game[0].len();
            game.resize_with(y as usize + 1, || vec![0; w]);
        }
        game[y as usize][x as usize] = id;
    }
    let cnt = game
        .iter()
        .flat_map(|row| row.iter())
        .filter(|v| **v == 2)
        .count();
    pv!(cnt);
}
