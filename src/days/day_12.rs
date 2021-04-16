use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/12.txt");

    let mut pos = input
        .lines()
        .map(|l| scanf!(l, "<x={}, y={}, z={}>", isize, isize, isize).unwrap())
        .to_vec();

    let mut vel = vec![(0, 0, 0); pos.len()];
    let mut cycle = (0, 0, 0);
    let mut states = (HashSet::new(), HashSet::new(), HashSet::new());

    for step in 0.. {
        if cycle.0 == 0
            && !states
                .0
                .insert(pos.iter().chain(vel.iter()).map(|p| p.0).to_vec())
        {
            cycle.0 = step;
            if cycle.0 != 0 && cycle.1 != 0 && cycle.2 != 0 {
                break;
            }
        }
        if cycle.1 == 0
            && !states
                .1
                .insert(pos.iter().chain(vel.iter()).map(|p| p.1).to_vec())
        {
            cycle.1 = step;
            if cycle.0 != 0 && cycle.1 != 0 && cycle.2 != 0 {
                break;
            }
        }
        if cycle.2 == 0
            && !states
                .2
                .insert(pos.iter().chain(vel.iter()).map(|p| p.2).to_vec())
        {
            cycle.2 = step;
            if cycle.0 != 0 && cycle.1 != 0 && cycle.2 != 0 {
                break;
            }
        }
        for (i, m1) in pos.iter().enumerate() {
            for (j, m2) in pos.iter().enumerate().skip(i + 1) {
                let delta = (m2.0 - m1.0).signum();
                vel[i].0 += delta;
                vel[j].0 -= delta;
                let delta = (m2.1 - m1.1).signum();
                vel[i].1 += delta;
                vel[j].1 -= delta;
                let delta = (m2.2 - m1.2).signum();
                vel[i].2 += delta;
                vel[j].2 -= delta;
            }
        }

        for (m, v) in pos.iter_mut().zip(vel.iter()) {
            m.0 += v.0;
            m.1 += v.1;
            m.2 += v.2;
        }
    }
    pv!(cycle);
    let lcm = num::integer::lcm(
        cycle.0 as u128,
        num::integer::lcm(cycle.1 as u128, cycle.2 as u128),
    );
    pv!(lcm);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/12.txt");

    let mut pos = input
        .lines()
        .map(|l| scanf!(l, "<x={}, y={}, z={}>", isize, isize, isize).unwrap())
        .to_vec();

    let mut vel = vec![(0, 0, 0); pos.len()];

    for _ in 0..1000 {
        for (i, m1) in pos.iter().enumerate() {
            for (j, m2) in pos.iter().enumerate().skip(i + 1) {
                let delta = (m2.0 - m1.0).signum();
                vel[i].0 += delta;
                vel[j].0 -= delta;
                let delta = (m2.1 - m1.1).signum();
                vel[i].1 += delta;
                vel[j].1 -= delta;
                let delta = (m2.2 - m1.2).signum();
                vel[i].2 += delta;
                vel[j].2 -= delta;
            }
        }

        for (m, v) in pos.iter_mut().zip(vel.iter()) {
            m.0 += v.0;
            m.1 += v.1;
            m.2 += v.2;
        }
    }

    let energy: isize = pos
        .iter()
        .zip(vel.iter())
        .map(|(m, v)| {
            let pot = m.0.abs() + m.1.abs() + m.2.abs();
            let kin = v.0.abs() + v.1.abs() + v.2.abs();
            pot * kin
        })
        .sum();
    pv!(energy);
}
