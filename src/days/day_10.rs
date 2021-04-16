use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/10.txt");

    let parsed = hashtag_grid(input);

    let max_radius = parsed.w().max(parsed.h());
    let mut max = 0;
    let mut max_v = (0, 0);
    let mut max_200 = 0;
    for (pos, &v) in parsed.grid_iter_index() {
        if !v {
            continue;
        }
        let mut field = parsed.clone();
        let mut visible = 0;
        let mut vaporize_order = vec![];
        for radius in 1..max_radius {
            for (mut next_pos, (dx, dy)) in parsed.square_ring_delta_iterator(pos, radius) {
                let mut angle = f32::atan2(dx as f32, -dy as f32);
                angle += std::f32::consts::TAU;
                angle %= std::f32::consts::TAU;
                let angle = (angle * 100_000.0) as isize;

                let mut order = 0;
                if field[next_pos] {
                    visible += 1;
                    vaporize_order.push((order, angle, next_pos));
                    order += 1;
                }
                while let Some(p) =
                    parsed.map_bounds((next_pos.0 as isize + dx, next_pos.1 as isize + dy))
                {
                    next_pos = p;
                    if field[next_pos] {
                        if order == 0 {
                            visible += 1;
                        }
                        vaporize_order.push((order, angle, next_pos));
                        order += 1;
                        field[next_pos] = false;
                    }
                }
            }
        }
        if visible > max {
            max = visible;
            max_v = pos;

            vaporize_order.sort_unstable();
            max_200 = vaporize_order[199].2 .0 * 100 + vaporize_order[199].2 .1;
        }
    }

    pv!(max_200);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/10.txt");

    let parsed = hashtag_grid(input);

    let max_radius = parsed.w().max(parsed.h());
    let mut max = 0;
    let mut max_v = (0, 0);
    for (pos, &v) in parsed.grid_iter_index() {
        if !v {
            continue;
        }
        let mut field = parsed.clone();
        let mut visible = 0;
        for radius in 1..max_radius {
            for (mut next_pos, (dx, dy)) in parsed.square_ring_delta_iterator(pos, radius) {
                let mut found = false;
                if field[next_pos] {
                    visible += 1;
                    found = true;
                }
                while let Some(p) =
                    parsed.map_bounds((next_pos.0 as isize + dx, next_pos.1 as isize + dy))
                {
                    next_pos = p;
                    if field[next_pos] {
                        if !found {
                            visible += 1;
                            found = true;
                        }
                        field[next_pos] = false;
                    }
                }
            }
        }
        if visible > max {
            max = visible;
            max_v = pos;
        }
    }

    pv!(max);
}
