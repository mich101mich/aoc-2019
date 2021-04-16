use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/11.txt");
    let mut code = IntProgram::new(input, vec![]);

    let mut hull = Grid::new_clone((200, 200), false);
    let mut dir = Dir::Up;
    let mut pos = (hull.w() / 2, hull.h() / 2);

    hull[pos] = true;
    loop {
        code.inputs.push(hull[pos] as isize);
        if let Some(color) = int_code(&mut code, true) {
            hull[pos] = color != 0;
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

    hull.trim();
    hull.print('#', ' ');
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/11.txt");
    let mut code = IntProgram::new(input, vec![]);

    let mut hull = Grid::new_clone((200, 200), false);
    let mut dir = Dir::Up;
    let mut pos = (hull.w() / 2, hull.h() / 2);
    let mut painted = HashSet::new();

    loop {
        code.inputs.push(hull[pos] as isize);
        if let Some(color) = int_code(&mut code, true) {
            hull[pos] = color != 0;
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
