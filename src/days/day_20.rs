use crate::utils::*;

fn find_in_maze(maze: &[Vec<char>], c: char) -> Vec<(usize, usize)> {
    maze.iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, v)| (x, y, v)))
        .filter(|(_, _, v)| **v == c)
        .map(|(x, y, _)| (x, y))
        .collect()
}

fn get_warp(
    maze: &[Vec<char>],
) -> (
    HashMap<(usize, usize), (usize, usize)>,
    HashMap<(usize, usize), (char, char)>,
) {
    let w = maze[0].len();
    let h = maze.len();
    let mut portals = HashSet::new();
    for c in (b'A'..=b'Z').map(|c| c as char) {
        for pos in find_in_maze(&maze, c) {
            let portal = {
                if pos.0 < w - 1 && maze[pos.1][pos.0 + 1].is_alphabetic() {
                    ((c, maze[pos.1][pos.0 + 1]), pos)
                } else if pos.1 < h - 1 && maze[pos.1 + 1][pos.0].is_alphabetic() {
                    ((c, maze[pos.1 + 1][pos.0]), pos)
                } else if pos.1 > 0 && maze[pos.1 - 1][pos.0].is_alphabetic() {
                    ((maze[pos.1 - 1][pos.0], c), (pos.0, pos.1 - 1))
                } else if pos.0 > 0 && maze[pos.1][pos.0 - 1].is_alphabetic() {
                    ((maze[pos.1][pos.0 - 1], c), (pos.0 - 1, pos.1))
                } else {
                    panic!("Missing Portal partner: {:?}", pos);
                }
            };
            portals.insert(portal);
        }
    }

    let mut portal_warps = HashMap::new();
    portal_warps.insert(('A', 'A'), vec![(0, 0)]);
    portal_warps.insert(('Z', 'Z'), vec![(1, 1)]);

    for &(portal, pos) in &portals {
        let wp = if pos.0 + 2 < maze[0].len() && maze[pos.1][pos.0 + 2] == '.' {
            (pos.0 + 2, pos.1)
        } else if pos.1 + 2 < maze.len() && maze[pos.1 + 2][pos.0] == '.' {
            (pos.0, pos.1 + 2)
        } else if pos.1 > 0 && maze[pos.1 - 1][pos.0] == '.' {
            (pos.0, pos.1 - 1)
        } else if pos.0 > 0 && maze[pos.1][pos.0 - 1] == '.' {
            (pos.0 - 1, pos.1)
        } else {
            panic!("Missing Portal entrance: {:?}", pos);
        };
        portal_warps
            .entry(portal)
            .or_insert_with(|| vec![])
            .push(wp);
    }

    let mut warp = HashMap::new();
    let mut warp_name = HashMap::new();
    for (name, wps) in portal_warps.iter() {
        warp.insert(wps[0], wps[1]);
        warp.insert(wps[1], wps[0]);
        warp_name.insert(wps[0], *name);
        warp_name.insert(wps[1], *name);
    }
    (warp, warp_name)
}

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/20.txt");
    // let input = include_str!("fi20.txt");
    let maze = input.lines().map(|l| l.chars().to_vec()).to_vec();
    let w = maze[0].len();
    let h = maze.len();

    let (warp, warp_name) = get_warp(&maze);

    let start = (warp[&(0, 0)], 0);
    let goal = (warp[&(1, 1)], 0);

    let mut visited = HashMap::new();
    let mut next = vec![(start, 0)];
    visited.insert(start, (0, start));

    let neigh = ManhattanNeighborhood::new(w, h);

    while let Some(((pos, level), _)) = next.pop() {
        let cost = visited[&(pos, level)].0;

        if (pos, level) == goal {
            pv!(cost);
            break;
        }

        let mut candidates = neigh
            .get_all_neighbors(pos)
            .filter(|&(x, y)| maze[y][x] == '.')
            .map(|p| (p, level))
            .to_vec();

        if let Some(&wp) = warp.get(&pos) {
            if wp == (0, 0) || wp == (1, 1) {
                // nope
            } else if pos.0 == 2 || pos.1 == 2 || pos.0 == w - 3 || pos.1 == h - 3 {
                // outer
                if level > 0 {
                    candidates.push((wp, level - 1));
                }
            } else {
                // inner
                candidates.push((wp, level + 1));
            }
        }

        for other_id in candidates {
            let other_cost = cost + 1;

            if let Some(&(prev_cost, _)) = visited.get(&other_id) {
                if prev_cost > other_cost {
                    next.retain(|&(id, _)| id != other_id);
                }
            }

            if !visited.contains_key(&other_id) || visited[&other_id].0 > other_cost {
                ordered_insert(&mut next, (other_id, other_cost), |&(_, cost)| cost);
                visited.insert(other_id, (other_cost, (pos, level)));
            }
        }
    }

}
