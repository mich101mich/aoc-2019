use crate::utils::*;

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/21.txt");

    let mut code = IntProgram::new(input, vec![]);
    let commands = vec![
        "NOT A J",
        "NOT B T",
        "OR T J",
        "NOT C T",
        "OR T J",
        "OR T T",
        "NOT T T",
        "OR H T",
        "OR E T",
        "AND T J",
        "AND D J",
        "RUN",
    ];
    // J = D & (!A | !B | !C) & (H | E)
    for command in commands {
        for c in command.chars() {
            code.inputs.push(c as u8 as isize);
        }
        code.inputs.push(b'\n' as isize);
    }
    while let Some(c) = int_code(&mut code, true) {
        if c > 255 {
            pv!(c);
            break;
        }
        print!("{}", c as u8 as char);
    }
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/21.txt");

    let mut code = IntProgram::new(input, vec![]);
    let commands = vec![
        "NOT A J", "NOT B T", "OR T J", "NOT C T", "OR T J", "AND D J", "WALK",
    ];
    // J = D & (!A | !B | !C)
    for command in commands {
        for c in command.chars() {
            code.inputs.push(c as u8 as isize);
        }
        code.inputs.push(b'\n' as isize);
    }
    while let Some(c) = int_code(&mut code, true) {
        if c > 255 {
            pv!(c);
            break;
        }
        print!("{}", c as u8 as char);
    }
}
