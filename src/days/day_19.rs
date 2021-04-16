use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/19.txt");

    let code = IntProgram::new(input, vec![]);

    let run = |x, y| {
        let mut code = code.clone();
        code.inputs.push(x);
        code.inputs.push(y);
        int_code(&mut code, true).unwrap() != 0
    };

    let x = binary_search_i(100, |x| {
        let mut y_start = 0;
        while !run(x, y_start) {
            y_start += 10;
        }
        let y_end = binary_search_i(y_start, |y| !run(x, y));
        run(x + 99, y_end - 100)
    });
    let mut y_start = 0;
    while !run(x, y_start) {
        y_start += 10;
    }
    let y_end = binary_search_i(y_start, |y| !run(x, y)) - 100;
    pv!(x * 10_000 + y_end);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/19.txt");

    let code = IntProgram::new(input, vec![]);
    let mut count = 0;
    for y in 0..50 {
        for x in 0..50 {
            let mut code = code.clone();
            code.inputs.push(x);
            code.inputs.push(y);
            let res = int_code(&mut code, true).unwrap();
            count += res;
            // print!("{}", res);
        }
        // println!();
    }
    pv!(count);
}
