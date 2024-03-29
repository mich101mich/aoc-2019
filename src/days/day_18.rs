use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/18.txt");

    const START: [u8; 4] = [27, 28, 29, 30];

    let mut maze = char_grid(input);

    let start = maze.find('@').unwrap();
    maze[start] = '#';
    for dir in Dir::all() {
        maze[start + dir] = '#';
    }
    let mut start_pos = [(0, 0); 4];
    for (i, (pos, &key)) in start_pos.iter_mut().zip(START.iter()).enumerate() {
        *pos = start + Dir::from(i) + Dir::from(i).clockwise();
    }

    let mut key_pos = HashMap::new();
    let mut door_pos = HashMap::new();
    for c in b'a'..=b'z' {
        if let Some(pos) = maze.find(c as char) {
            key_pos.insert(pos, c - b'a');
        }
        if let Some(pos) = maze.find((c - b'a' + b'A') as char) {
            door_pos.insert(pos, c - b'a');
        }
    }

    let goal_keys = key_pos.values().fold(0u32, |total, v| total | (1 << v));

    let mut costs = HashMap::new();

    for (&pos, &key) in key_pos.iter().chain(start_pos.iter().zip(START.iter())) {
        let targets = key_pos.keys().copied().filter(|p| *p != pos).to_vec();
        let found = maze.dijkstra(pos, &targets, |c| *c != '#');
        let mut map = HashMap::new();

        for (pos, path) in found {
            let prec = path
                .path
                .iter()
                .filter_map(|p| door_pos.get(p))
                .fold(0u32, |total, v| total | (1 << v));

            map.insert(key_pos[&pos], (prec, path.cost));
        }

        costs.insert(key, map);
    }

    let goal = (START, goal_keys);

    let num_keys = key_pos.len();

    let path = a_star_search(
        |(pos_arr, keys), out| {
            if keys == goal_keys {
                out.push((goal, 0));
                return;
            }
            for (i, pos) in pos_arr.iter().enumerate() {
                for (k, (pred, cost)) in costs[pos].iter() {
                    if keys & (1 << *k) == 0 && pred & !keys == 0 {
                        let mut new_pos = pos_arr;
                        new_pos[i] = *k;
                        out.push(((new_pos, keys | (1 << k)), *cost));
                    }
                }
            }
        },
        (START, 0),
        goal,
        |(_, keys)| num_keys - keys.count_ones() as usize,
    );

    pv!(path.unwrap().cost);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/18.txt");

    const START: u8 = 30;

    let mut maze = char_grid(input);

    let start = maze.find('@').unwrap();
    maze[start] = '.';

    let mut key_pos = HashMap::new();
    let mut door_pos = HashMap::new();
    for c in b'a'..=b'z' {
        if let Some(pos) = maze.find(c as char) {
            key_pos.insert(pos, c - b'a');
        }
        if let Some(pos) = maze.find((c - b'a' + b'A') as char) {
            door_pos.insert(pos, c - b'a');
        }
    }
    let goal_keys = key_pos.values().fold(0u32, |total, v| total | (1 << v));

    let mut costs = HashMap::new();

    for (&pos, &key) in key_pos.iter().chain(std::iter::once((&start, &START))) {
        let targets = key_pos.keys().copied().filter(|p| *p != pos).to_vec();
        let found = maze.dijkstra(pos, &targets, |c| *c != '#');
        let mut map = HashMap::new();

        for (pos, path) in found {
            let prec = path
                .path
                .iter()
                .filter_map(|p| door_pos.get(p))
                .fold(0u32, |total, v| total | (1 << v));
            map.insert(key_pos[&pos], (prec, path.cost));
        }

        if key != START {
            map.insert(START, (goal_keys, 0));
        }

        costs.insert(key, map);
    }

    let path = a_star_search(
        |(pos, keys), out| {
            for (k, (pred, cost)) in costs[&pos].iter() {
                if keys & (1 << *k) == 0 && pred & !keys == 0 {
                    out.push(((*k, keys | (1 << k)), *cost));
                }
            }
        },
        (START, 0),
        (START, goal_keys | (1 << START)),
        |_| 0,
    );

    pv!(path.unwrap().cost);
}
