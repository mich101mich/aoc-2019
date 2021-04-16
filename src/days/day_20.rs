use crate::utils::*;

type PathList = Vec<(Point, usize)>;

fn portal_graph(maze: &Grid<char>) -> (HashMap<Point, PathList>, Point, Point) {
    let mut single_portals = HashMap::new();
    let mut all_paths = HashMap::new();
    let mut start = (0, 0);
    let mut goal = (0, 0);
    for (pos, c) in maze.grid_iter_index() {
        if !('A'..='Z').contains(c) {
            continue;
        }
        for &dir in [Dir::Right, Dir::Down].iter() {
            if !maze
                .get(pos + dir)
                .map(|c| ('A'..='Z').contains(c))
                .unwrap_or(false)
            {
                continue;
            }
            let portal_pos = if maze.get(pos - dir) == Some(&'.') {
                pos - dir
            } else {
                assert_eq!(maze.get(pos + dir + dir), Some(&'.'));
                pos + dir + dir
            };
            let portal = (maze[pos], maze[pos + dir]);
            if let Some(other_pos) = single_portals.remove(&portal) {
                all_paths.insert(portal_pos, vec![(other_pos, 1)]);
                all_paths.insert(other_pos, vec![(portal_pos, 1)]);
            } else if portal == ('A', 'A') {
                start = portal_pos;
                all_paths.insert(portal_pos, vec![]);
            } else if portal == ('Z', 'Z') {
                goal = portal_pos;
                all_paths.insert(portal_pos, vec![]);
            } else {
                single_portals.insert(portal, portal_pos);
            }
            break;
        }
    }

    assert_eq!(single_portals.len(), 0);

    let portals = all_paths.keys().copied().to_vec();

    for (&pos, paths) in all_paths.iter_mut() {
        let reachable = maze.dijkstra(pos, &portals, |c| *c == '.');
        for (other, path) in reachable {
            if other != pos {
                paths.push((other, path.cost));
            }
        }
    }

    (all_paths, start, goal)
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/20.txt");

    let maze = char_grid(input);
    let (w, h) = maze.bounds();

    let (graph, start, goal) = portal_graph(&maze);
    let start = ((start.0, start.1), 0usize);
    let goal = ((goal.0, goal.1), 0);

    let neigh = maze.manhattan();
    let path = a_star_search(
        |(p, l)| {
            graph[&p].iter().filter_map(move |&(pos, cost)| {
                if cost == 1 {
                    let outer = p.0 < 5 || p.1 < 5 || w - p.0 < 5 || h - p.1 < 5;
                    if outer && l == 0 {
                        None
                    } else {
                        Some(((pos, if outer { l - 1 } else { l + 1 }), cost))
                    }
                } else {
                    Some(((pos, l), cost))
                }
            })
        },
        start,
        goal,
        |_| 0,
    )
    .unwrap();

    pv!(path.cost);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/20.txt");

    let maze = char_grid(input);

    let (graph, start, goal) = portal_graph(&maze);

    let neigh = maze.manhattan();
    let path = a_star_search(|p| graph[&p].iter(), start, goal, |_| 0).unwrap();

    pv!(path.cost);
}
