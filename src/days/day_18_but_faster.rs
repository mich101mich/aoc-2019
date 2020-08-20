mod utils;
use std::collections::VecDeque;
use utils::*;

fn find_in_maze(maze: &[Vec<char>], c: char) -> Vec<(usize, usize)> {
    maze.iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, v)| (x, y, v)))
        .filter(|(_, _, v)| **v == c)
        .map(|(x, y, _)| (x, y))
        .collect()
}

pub fn main() {
    #[allow(unused_variables)]
    let input = include_str!("18.txt");

    // 4954

    let mut maze = input.lines().map(|l| l.chars().to_vec()).to_vec();

    let start = find_in_maze(&maze, '@')[0];
    maze[start.1][start.0] = '.';

    let keys_pos: HashMap<(usize, usize), u8> = maze
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, v)| (x, y, *v)))
        .filter(|(_, _, v)| v.is_alphabetic() && v.is_lowercase())
        .map(|(x, y, v)| ((x, y), v as u8 - b'a'))
        .collect();

    let door_pos: HashMap<(usize, usize), u8> = maze
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, v)| (x, y, *v)))
        .filter(|(_, _, v)| v.is_alphabetic() && v.is_uppercase())
        .map(|(x, y, v)| ((x, y), v as u8 - b'A'))
        .collect();

    let neigh = ManhattanNeighborhood::new(maze[0].len(), maze.len());
    let mut costs: HashMap<u8, HashMap<u8, (u32, usize)>> = HashMap::new();
    for (&pos, &key) in keys_pos.iter() {
        let targets = keys_pos.keys().copied().filter(|p| *p != pos).to_vec();
        let found = dijkstra_search(
            |p| neigh.get_all_neighbors(p),
            |_, _| 1,
            |(x, y)| maze[y][x] != '#',
            pos,
            &targets,
        );
        let mut map = HashMap::new();

        for (pos, path) in found {
            let prec = path
                .path
                .iter()
                .filter_map(|p| door_pos.get(p))
                .fold(0, |total, v| total | (1 << v));
            map.insert(keys_pos[&pos], (prec, path.cost));
        }

        costs.insert(key, map);
    }
    {
        let starts = dijkstra_search(
            |p| neigh.get_all_neighbors(p),
            |_, _| 1,
            |(x, y)| maze[y][x] == '.',
            start,
            &keys_pos.keys().copied().to_vec()[..],
        );
        let mut map = HashMap::new();
        for (pos, path) in starts {
            map.insert(keys_pos[&pos], (0, path.cost));
        }
        costs.insert(50, map);
    }

    #[derive(Debug, PartialEq, Eq)]
    struct State {
        pos: u8,
        keys: u32,
        steps: usize,
    }
    impl std::cmp::Ord for State {
        fn cmp(&self, rhs: &State) -> std::cmp::Ordering {
            rhs.steps.cmp(&self.steps)
        }
    }
    impl std::cmp::PartialOrd for State {
        fn partial_cmp(&self, rhs: &State) -> Option<std::cmp::Ordering> {
            Some(self.cmp(rhs))
        }
    }

    let mut seen = HashMap::<(u8, u32), usize>::with_capacity(50_000);

    fn dedup_insert(vector: &mut VecDeque<State>, element: State) {
        let mut index = None;
        for (i, other) in vector.iter_mut().enumerate() {
            if other.pos == element.pos && other.keys == element.keys {
                other.steps = other.steps.min(element.steps);
                return;
            }
            if other.steps >= element.steps {
                index = Some(i);
                break;
            }
        }
        if let Some(i) = index {
            let mut j = i;
            while j < vector.len() {
                if vector[j].pos == element.pos && vector[j].keys == element.keys {
                    vector.remove(j);
                    break;
                }
                j += 1;
            }
            vector.insert(i, element);
        } else {
            vector.push_back(element);
        }
    }

    let mut next = VecDeque::with_capacity(50_000);
    next.push_back(State {
        pos: 50,
        keys: 0,
        steps: 0,
    });

    let mut cnt = 0;
    while let Some(State { pos, keys, steps }) = next.pop_front() {
        if let Some(shortest_steps) = seen.get(&(pos, keys)) {
            if *shortest_steps < steps {
                continue;
            }
        }

        cnt += 1;
        if cnt > 1_000 {
            println!("{} {} {}", steps, next.len(), seen.len());
            cnt = 0;
        }
        let candidates = costs[&pos]
            .iter()
            .filter(|(k, _)| keys & (1 << **k) == 0)
            .filter(|(_, (prec, _))| prec & !keys == 0);

        let mut found = false;
        for (k, (_, cost)) in candidates {
            found = true;
            let state = State {
                pos: *k,
                keys: keys | (1 << *k),
                steps: steps + cost,
            };
            if let Some(steps) = seen.get_mut(&(state.pos, state.keys)) {
                if *steps < state.steps {
                    continue;
                }
                *steps = state.steps;
                dedup_insert(&mut next, state)
            } else {
                seen.insert((state.pos, state.keys), state.steps);
                let mut a = 0;
                let mut b = next.len();
                while b - a > 1 {
                    let mid = (a + b) / 2;
                    if next[mid].steps > state.steps {
                        b = mid;
                    } else {
                        a = mid;
                    }
                }
                next.insert(a, state);
            }
        }

        if !found {
            pv!(steps);
            return;
        }
    }
}
