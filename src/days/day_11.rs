use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/11.txt");
    // let input = ;
    let mut code = IntProgram::new(input, vec![]);

    let mut hull = vec![vec![0; 100]; 100];
    let mut dir = Dir::Up;
    let mut pos = (50, 50);

    hull[pos.1][pos.0] = 1;
    loop {
        code.inputs.push(hull[pos.1][pos.0]);
        if let Some(color) = int_code(&mut code, true) {
            hull[pos.1][pos.0] = color;
        } else {
            break;
        }
        let dir_change = int_code(&mut code, true).unwrap();
        if dir_change == 0 {
            dir = dir.counter_clockwise();
        } else {
            dir = dir.clockwise();
        }
        pos += dir;
    }

    trim_grid(&mut hull, |x| *x == 0);

    for row in &hull {
        for color in row {
            if *color == 1 {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/11.txt");
    // let input = ;
    let mut code = IntProgram::new(input, vec![]);

    let mut hull = vec![vec![0; 200]; 200];
    let mut dir = Dir::Up;
    let mut pos = (100, 100);
    let mut painted = HashSet::new();

    loop {
        code.inputs.push(hull[pos.1][pos.0]);
        if let Some(color) = int_code(&mut code, true) {
            hull[pos.1][pos.0] = color;
            painted.insert(pos);
        } else {
            break;
        }
        let dir_change = int_code(&mut code, true).unwrap();
        if dir_change == 0 {
            dir = dir.counter_clockwise();
        } else {
            dir = dir.clockwise();
        }
        pos += dir;
    }
    pv!(painted.len());
}
