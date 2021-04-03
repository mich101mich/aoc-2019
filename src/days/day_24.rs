use crate::utils::*;

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/24.txt");
// 	let input = "....#
// #..#.
// #..##
// ..#..
// #....";

    let mut grid = [['.'; 5]; 5];
    for (y, row) in input.lines().enumerate() {
        for (x, v) in row.chars().enumerate() {
            grid[y][x] = v;
        }
    }

    let mut state = HashMap::new();
    state.insert(0i32, grid);
    let mut min_level = 0;
    let mut max_level = 0;

    for _ in 0..200 {
        if state[&min_level]
            .iter()
            .flat_map(|r| r.iter())
            .any(|v| *v == '#')
        {
            min_level -= 1;
            state.insert(min_level, [['.'; 5]; 5]);
        }
        if state[&max_level]
            .iter()
            .flat_map(|r| r.iter())
            .any(|v| *v == '#')
        {
            max_level += 1;
            state.insert(max_level, [['.'; 5]; 5]);
        }
        let mut next = state.clone();

        for (layer, grid) in state.iter() {
            let next = next.get_mut(layer).unwrap();
            for y in 0..5 {
                for x in 0..5 {
                    if y == 2 && x == 2 {
                        continue;
                    }
                    let mut count = 0;

                    // up
                    if y == 0 {
                        count += state
                            .get(&(layer - 1))
                            .map(|l| (l[1][2] == '#') as usize)
                            .unwrap_or(0);
                    } else if y == 3 && x == 2 {
                        count += state
                            .get(&(layer + 1))
                            .map(|l| l[4].iter().filter(|v| **v == '#').count())
                            .unwrap_or(0);
                    } else {
                        count += (grid[y - 1][x] == '#') as usize;
                    }

                    // down
                    if y == 4 {
                        count += state
                            .get(&(layer - 1))
                            .map(|l| (l[3][2] == '#') as usize)
                            .unwrap_or(0);
                    } else if y == 1 && x == 2 {
                        count += state
                            .get(&(layer + 1))
                            .map(|l| l[0].iter().filter(|v| **v == '#').count())
                            .unwrap_or(0);
                    } else {
                        count += (grid[y + 1][x] == '#') as usize;
                    }

                    // right
                    if x == 4 {
                        count += state
                            .get(&(layer - 1))
                            .map(|l| (l[2][3] == '#') as usize)
                            .unwrap_or(0);
                    } else if y == 2 && x == 1 {
                        count += state
                            .get(&(layer + 1))
                            .map(|l| l.iter().map(|r| r[0]).filter(|v| *v == '#').count())
                            .unwrap_or(0);
                    } else {
                        count += (grid[y][x + 1] == '#') as usize;
                    }

                    // left
                    if x == 0 {
                        count += state
                            .get(&(layer - 1))
                            .map(|l| (l[2][1] == '#') as usize)
                            .unwrap_or(0);
                    } else if y == 2 && x == 3 {
                        count += state
                            .get(&(layer + 1))
                            .map(|l| l.iter().map(|r| r[4]).filter(|v| *v == '#').count())
                            .unwrap_or(0);
                    } else {
                        count += (grid[y][x - 1] == '#') as usize;
                    }

                    if grid[y][x] == '#' && count != 1 {
                        next[y][x] = '.';
                    } else if grid[y][x] == '.' && (count == 1 || count == 2) {
                        next[y][x] = '#';
                    }
                }
            }
        }

        state = next;
    }

    let count = state
        .values()
        .flat_map(|g| g.iter())
        .flat_map(|r| r.iter())
        .filter(|c| **c == '#')
        .count();
    pv!(count);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/24.txt");

    let mut eris = input.lines().map(|l| l.chars().to_vec()).to_vec();
    let neigh = ManhattanNeighborhood::new(5, 5);

    let mut states = HashSet::new();

    loop {
        let state = eris.iter().flat_map(|r| r.iter()).collect::<String>();
        if !states.insert(state) {
            pv!(eris);
            break;
        }
        let mut next = eris.clone();
        for y in 0..5 {
            for x in 0..5 {
                let count = neigh
                    .get_all_neighbors((x, y))
                    .filter(|p| eris[p.1][p.0] == '#')
                    .count();
                if eris[y][x] == '#' && count != 1 {
                    next[y][x] = '.';
                } else if eris[y][x] == '.' && (count == 1 || count == 2) {
                    next[y][x] = '#';
                }
            }
        }
        eris = next;
    }

    let mut bio = 0;
    for y in 0..5 {
        for x in 0..5 {
            if eris[y][x] == '#' {
                bio += 2usize.pow((y * 5 + x) as u32);
            }
        }
    }
    pv!(bio);
}
