use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/24.txt");

    let ground = hashtag_grid(input);
    let size = ground.bounds();
    let neigh = ground.manhattan();
    let mut eris = vec![ground];

    let mut next = eris.clone();

    for _ in 0..200 {
        if eris[0].grid_iter().any(|b| *b) {
            eris.insert(0, Grid::new_clone(size, false));
            next.insert(0, Grid::new_clone(size, false));
        }
        if eris.last().unwrap().grid_iter().any(|b| *b) {
            eris.push(Grid::new_clone(size, false));
            next.push(Grid::new_clone(size, false));
        }

        for ((layer, cur), next) in eris.iter().enumerate().zip(next.iter_mut()) {
            for ((p, v), n) in cur.grid_iter_index().zip(next.grid_iter_mut()) {
                if p == (2, 2) {
                    continue;
                }
                let mut count = 0;
                for d in Dir::all() {
                    let neigh = p + d;
                    if neigh == (2, 2) {
                        if layer == eris.len() - 1 {
                            continue;
                        }
                        for v in eris[layer + 1].border(d.opposite()) {
                            count += *v as usize;
                        }
                    } else if let Some(v) = cur.get(neigh) {
                        count += *v as usize;
                    } else {
                        if layer == 0 {
                            continue;
                        }
                        let v = eris[layer - 1][(2usize, 2) + d];
                        count += v as usize;
                    }
                }
                *n = count == 1 || (!*v && count == 2);
            }
        }

        std::mem::swap(&mut eris, &mut next);
    }

    let count = eris
        .iter()
        .flat_map(|layer| layer.grid_iter())
        .filter(|c| **c)
        .count();
    pv!(count);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/24.txt");

    let mut eris = hashtag_grid(input);
    let neigh = eris.manhattan();

    let mut states = HashSet::new();
    let mut next = eris.clone();

    loop {
        let state = eris
            .grid_iter()
            .fold(0u32, |total, b| (total << 1) | (*b as u32));
        if !states.insert(state) {
            break;
        }
        for ((p, v), n) in eris.grid_iter_index().zip(next.grid_iter_mut()) {
            let count = neigh.get_all_neighbors(p).filter(|p| eris[*p]).count();
            *n = count == 1 || (!*v && count == 2);
        }
        std::mem::swap(&mut eris, &mut next);
    }

    let mut bio = 0;
    let mut pow = 1;
    for v in eris.grid_iter() {
        if *v {
            bio += pow;
        }
        pow *= 2;
    }
    pv!(bio);
}
