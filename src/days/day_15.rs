use crate::utils::*;

fn dir_to_input(dir: Dir) -> isize {
    match dir {
        Dir::Up => 1,
        Dir::Down => 2,
        Dir::Left => 3,
        Dir::Right => 4,
    }
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/15.txt");

    let mut code = IntProgram::new(input, vec![]);

    let mut area = HashMap::new();
    let mut pos = (0, 0isize);
    let mut remaining = HashSet::new();
    remaining.insert(pos);
    let mut start = (0, 0);

    loop {
        remaining.remove(&pos);
        let mut next_dir = None;

        for dir in Dir::all() {
            let new_pos = pos + dir;
            if area.contains_key(&new_pos) {
                continue;
            }
            code.inputs.push(dir_to_input(dir));
            let res = int_code(&mut code, true).unwrap();
            match res {
                0 => {
                    area.insert(new_pos, '#');
                }
                1 => {
                    area.insert(new_pos, '.');
                    remaining.insert(new_pos);
                    next_dir = Some(dir);

                    code.inputs.push(dir_to_input(dir.opposite()));
                    assert_ne!(int_code(&mut code, true).unwrap(), 0);
                }
                2 => {
                    area.insert(new_pos, '.');
                    remaining.insert(new_pos);
                    next_dir = Some(dir);

                    start = new_pos;

                    code.inputs.push(dir_to_input(dir.opposite()));
                    assert_eq!(int_code(&mut code, true).unwrap(), 1);
                }
                x => panic!("Invalid response: {}", x),
            }
        }
        if let Some(next_dir) = next_dir {
            code.inputs.push(dir_to_input(next_dir));
            assert_ne!(int_code(&mut code, true).unwrap(), 0);
            pos += next_dir;
        } else if remaining.is_empty() {
            break;
        } else {
            let goal = *remaining.iter().next().unwrap();
            let path = a_star_search(
                |pos, out| {
                    for d in Dir::all() {
                        let p = pos + d;
                        if area.get(&p) == Some(&'.') {
                            out.push(p);
                        }
                    }
                },
                pos,
                goal,
                |p| manhattan_i(p, goal) as usize,
            )
            .unwrap();
            for p in path.path.iter().skip(1) {
                let dir = Dir::from_difference(pos, *p);
                assert_eq!(pos + dir, *p);
                code.inputs.push(dir_to_input(dir));
                assert_ne!(int_code(&mut code, true).unwrap(), 0);
                pos += dir;
            }
        }
    }

    let goals = area
        .iter()
        .filter(|(_, c)| **c == '.')
        .map(|(p, _)| *p)
        .to_vec();

    let paths = dijkstra_search(
        |pos, out| {
            for d in Dir::all() {
                let p = pos + d;
                if area.get(&p) == Some(&'.') {
                    out.push(p);
                }
            }
        },
        start,
        &goals,
    );

    let longest = paths.values().map(|p| p.cost).max().unwrap();

    pv!(longest);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/15.txt");

    let mut code = IntProgram::new(input, vec![]);

    let mut area = HashMap::new();
    let mut pos = (0, 0isize);
    let mut remaining = HashSet::new();
    remaining.insert(pos);
    let mut goal = (0, 0);

    loop {
        remaining.remove(&pos);
        let mut next_dir = None;

        for dir in Dir::all() {
            let new_pos = pos + dir;
            if area.contains_key(&new_pos) {
                continue;
            }
            code.inputs.push(dir_to_input(dir));
            let res = int_code(&mut code, true).unwrap();
            match res {
                0 => {
                    area.insert(new_pos, '#');
                }
                1 => {
                    area.insert(new_pos, '.');
                    remaining.insert(new_pos);
                    next_dir = Some(dir);

                    code.inputs.push(dir_to_input(dir.opposite()));
                    assert_ne!(int_code(&mut code, true).unwrap(), 0);
                }
                2 => {
                    area.insert(new_pos, '.');
                    remaining.insert(new_pos);
                    next_dir = Some(dir);

                    goal = new_pos;

                    code.inputs.push(dir_to_input(dir.opposite()));
                    assert_eq!(int_code(&mut code, true).unwrap(), 1);
                }
                x => panic!("Invalid response: {}", x),
            }
        }
        if let Some(next_dir) = next_dir {
            code.inputs.push(dir_to_input(next_dir));
            assert_ne!(int_code(&mut code, true).unwrap(), 0);
            pos += next_dir;
        } else if remaining.is_empty() {
            break;
        } else {
            let goal = *remaining.iter().next().unwrap();
            let path = a_star_search(
                |pos, out| {
                    for d in Dir::all() {
                        let p = pos + d;
                        if area.get(&p) == Some(&'.') {
                            out.push(p);
                        }
                    }
                },
                pos,
                goal,
                |p| manhattan_i(p, goal) as usize,
            )
            .unwrap();
            for p in path.path.iter().skip(1) {
                let dir = Dir::from_difference(pos, *p);
                assert_eq!(pos + dir, *p);
                code.inputs.push(dir_to_input(dir));
                assert_ne!(int_code(&mut code, true).unwrap(), 0);
                pos += dir;
            }
        }
    }

    {
        let left = area.keys().map(|p| p.0).min().unwrap();
        let top = area.keys().map(|p| p.1).min().unwrap();
        let right = area.keys().map(|p| p.0).max().unwrap();
        let bottom = area.keys().map(|p| p.1).max().unwrap();
        for y in top..=bottom {
            for x in left..=right {
                print!("{}", area.get(&(x, y)).unwrap_or(&' '));
            }
            println!();
        }
        println!();
    }

    let path = a_star_search(
        |pos, out| {
            for d in Dir::all() {
                let p = pos + d;
                if area.get(&p) == Some(&'.') {
                    out.push(p);
                }
            }
        },
        (0, 0),
        goal,
        |p| manhattan_i(p, goal) as usize,
    )
    .unwrap();
    pv!(path.cost);
}
