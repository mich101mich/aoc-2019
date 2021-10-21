use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/17.txt");

    let mut code = IntProgram::new(input, vec![]);

    let mut grid = Grid::new();
    grid.push(vec![]);

    let mut pos = (0, 0);
    let mut dir = Dir::Up;
    while let Some(c) = int_code(&mut code, true) {
        let c = c as u8 as char;
        match c {
            '\n' => {
                grid.push(vec![]);
            }
            '>' | '<' | '^' | 'v' => {
                grid.last_mut().unwrap().push(true);
                pos = (
                    grid.last().unwrap().len() as isize - 1,
                    grid.len() as isize - 1,
                );
                dir = c.into();
            }
            c => grid.last_mut().unwrap().push(c == '#'),
        }
    }
    grid.pop();
    grid.pop();

    let mut current = 0;
    let mut turn = "_";
    let mut path = vec![];
    loop {
        pos += dir;
        if grid.in_bounds(pos) && grid[pos] {
            current += 1;
            continue;
        }
        if current != 0 {
            path.push(turn.to_string());
            path.push(format!("{}", current));
        }
        current = 0;
        pos -= dir;
        dir = dir.clockwise();
        if grid.in_bounds(pos + dir) && grid[pos + dir] {
            turn = "R";
            continue;
        }
        dir = dir.opposite();
        if grid.in_bounds(pos + dir) && grid[pos + dir] {
            turn = "L";
            continue;
        }
        break;
    }
    let path_str = path.join(",");
    let path_str = path_str.as_str();
    pv!(path_str);

    let (mut a_str, mut b_str, mut c_str) = (path_str, path_str, path_str);

    let path_array = vec![path_str];

    fn multi_split<'a>(src: &[&'a str], split: &str, separator: char) -> Vec<&'a str> {
        src.iter()
            .flat_map(|v| v.split(split))
            .map(|s| s.trim_matches(separator))
            .filter(|s| !s.is_empty())
            .to_vec()
    }

    fn recurse<'a>(path_array: &[&'a str], sub_routines: &mut [&mut &'a str]) -> bool {
        let (my_str, sub_routines) = sub_routines.split_first_mut().unwrap();
        let path_str = &path_array[0];
        let max_len = path_str.len().min(20);
        for my_len in (2..=max_len).rev() {
            if my_len != path_str.len() && path_str.chars().nth(my_len).unwrap() != ',' {
                continue;
            }
            let slice = &path_str[..my_len];
            let path_array = multi_split(path_array, slice, ',');
            if sub_routines.is_empty() {
                if path_array.is_empty() {
                    **my_str = slice;
                    return true;
                }
            } else if recurse(&path_array, sub_routines) {
                **my_str = slice;
                return true;
            }
        }
        false
    }

    recurse(&path_array, &mut [&mut a_str, &mut b_str, &mut c_str]);
    let routine = path_str
        .replace(a_str, "A")
        .replace(b_str, "B")
        .replace(c_str, "C");

    code = IntProgram::new(input, vec![]);
    code.mem[0] = 2;

    for s in [&routine, a_str, b_str, c_str].iter() {
        pv!(s);
        code.add_input(s);
        code.inputs.push(b'\n' as isize);
    }

    // code.add_input("y\n"); // show output
    code.add_input("n\n");
    // let mut last = b'\n' as isize;
    while let Some(c) = int_code(&mut code, true) {
        if c < 256 {
            // print!("{}", c as u8 as char);
            // if c == b'\n' as isize && last == c {
            //     // std::thread::sleep(std::time::Duration::from_millis(200));
            // }
            // last = c;
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

    let mut grid = Grid::new();
    grid.push(vec![]);

    let mut pos = (0, 0);
    while let Some(c) = int_code(&mut code, true) {
        let c = c as u8 as char;
        match c {
            '\n' => {
                grid.push(vec![]);
            }
            '>' | '<' | '^' | 'v' => {
                grid.last_mut().unwrap().push(true);
                pos = (grid.last().unwrap().len() - 1, grid.len() - 1);
            }
            c => grid.last_mut().unwrap().push(c == '#'),
        }
    }
    grid.pop();
    grid.pop();

    let mut sum = 0;
    let neigh = grid.manhattan();

    for (pos, v) in grid.grid_iter_index() {
        if *v && neigh.get_all_neighbors(pos).all(|p| grid[p]) {
            sum += pos.0 * pos.1;
        }
    }
    // grid.print_hashtag();
    pv!(sum);
}
