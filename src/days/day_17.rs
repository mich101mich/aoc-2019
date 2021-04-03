use crate::utils::*;

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/17.txt");

    let mut code = IntProgram::new(input, vec![]);

    code.mem[0] = 2;

    let _total = "R,8,L,10,R,8,R,12,R,8,L,8,L,12,R,8,L,10,R,8,L,12,L,10,L,8,R,8,L,10,R,8,R,12,R,8,L,8,L,12,L,12,L,10,L,8,L,12,L,10,L,8,R,8,L,10,R,8,R,12,R,8,L,8,L,12";
    //            R,8,L,10,R,8                   R,8,L,10,R,8               R,8,L,10,R,8                                               R,8,L,10,R,8                  
    //                         R,12,R,8,L,8,L,12                                         R,12,R,8,L,8,L,12                                          R,12,R,8,L,8,L,12
    //                                                        L,12,L,10,L,8                                L,12,L,10,L,8 L,12,L,10,L,8                               

    let routines = "A,B,A,C,A,B,C,C,A,B\n";
    let a = "R,8,L,10,R,8\n";
    let b = "R,12,R,8,L,8,L,12\n";
    let c = "L,12,L,10,L,8\n";

    for c in routines.chars() {
        code.inputs.push(c as u8 as isize);
    }
    for c in a.chars() {
        code.inputs.push(c as u8 as isize);
    }
    for c in b.chars() {
        code.inputs.push(c as u8 as isize);
    }
    for c in c.chars() {
        code.inputs.push(c as u8 as isize);
    }
    code.inputs.push(b'y' as isize);
    code.inputs.push(b'\n' as isize);

    let mut last = b'\n' as isize;
    while let Some(c) = int_code(&mut code, true) {
        if c < 256 {
            print!("{}", c as u8 as char);
            if c == b'\n' as isize && last == c {
                // std::thread::sleep(std::time::Duration::from_millis(200));
            }
            last = c;
        } else {
            pv!(c);
        }
    }
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/17.txt");

    let mut code = IntProgram::new(input, vec![]);

    let mut area = vec![vec![]];

    while let Some(c) = int_code(&mut code, true) {
        let c = c as u8 as char;
        if c != '\n' {
            area.last_mut().unwrap().push(c);
        } else {
            area.push(vec![]);
        }
    }
    area.pop();
    area.pop();

    let mut sum = 0;

    for y in 1..area.len() - 1 {
        for x in 1..area[0].len() - 1 {
            if area[y][x] == '#'
                && area[y - 1][x] == '#'
                && area[y + 1][x] == '#'
                && area[y][x - 1] == '#'
                && area[y][x + 1] == '#'
            {
                sum += y * x;
            }
        }
    }
    for row in area.iter() {
        for v in row.iter() {
            print!("{}", v);
        }
        println!();
    }
    pv!(sum);
}
