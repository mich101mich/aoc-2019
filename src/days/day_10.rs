use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/10.txt");

    let parsed = hashtag_grid(input);

    let size = (parsed.len(), parsed[0].len());
    let mut max = 0;
    let mut max_v = (0, 0);
    let mut max_200 = 0;
    for (x, y) in grid_iterator(size) {
        if !parsed[y][x] {
            continue;
        }
        let mut field = parsed.clone();
        let mut visible = 0;
        let mut vaporize_order = vec![];
        for radius in 1..size.0.max(size.1) {
            for ((mut nx, mut ny), (dx, dy)) in square_ring_delta_iterator((x, y), radius, size) {
                let mut angle = f32::atan2(dx as f32, -dy as f32);
                angle += std::f32::consts::TAU;
                angle %= std::f32::consts::TAU;
                let angle = (angle * 100_000.0) as isize;

                let mut order = 0;
                if field[ny][nx] {
                    visible += 1;
                    vaporize_order.push((order, angle, nx, ny));
                    order += 1;
                }
                while let Some(p) = map_grid_bounds((nx as isize + dx, ny as isize + dy), size) {
                    nx = p.0;
                    ny = p.1;
                    if field[ny][nx] {
                        if order == 0 {
                            visible += 1;
                        }
                        vaporize_order.push((order, angle, nx, ny));
                        order += 1;
                        field[ny][nx] = false;
                    }
                }
            }
        }
        if visible > max {
            max = visible;
            max_v = (x, y);

            vaporize_order.sort_unstable();
            max_200 = vaporize_order[199].2 * 100 + vaporize_order[199].3;
        }
    }

    pv!(max_200);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/10.txt");

    let parsed = hashtag_grid(input);

    let size = (parsed.len(), parsed[0].len());
    let mut max = 0;
    let mut max_v = (0, 0);
    for (x, y) in grid_iterator(size) {
        if !parsed[y][x] {
            continue;
        }
        let mut field = parsed.clone();
        let mut visible = 0;
        for radius in 1..size.0.max(size.1) {
            for ((mut nx, mut ny), (dx, dy)) in square_ring_delta_iterator((x, y), radius, size) {
                let mut found = false;
                if field[ny][nx] {
                    visible += 1;
                    found = true;
                }
                while let Some(p) = map_grid_bounds((nx as isize + dx, ny as isize + dy), size) {
                    nx = p.0;
                    ny = p.1;
                    if field[ny][nx] {
                        if !found {
                            visible += 1;
                            found = true;
                        }
                        field[ny][nx] = false;
                    }
                }
            }
        }
        if visible > max {
            max = visible;
            max_v = (x, y);
        }
    }

    pv!(max);
    pv!(max_v);
}
