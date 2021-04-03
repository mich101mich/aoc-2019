use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/03.txt");

    let parsed = input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|s| scanf!(s, "{}{}", Dir, isize).unwrap())
                .to_vec()
        })
        .to_vec();

    let mut positions = HashSet::new();
    let mut pos = (0isize, 0);
    let mut distances = HashMap::new();
    let mut steps = 0;
    for (dir, n) in parsed[0].iter().copied() {
        for _ in 0..n {
            pos += dir;
            positions.insert(pos);
            steps += 1;
            distances.entry(pos).or_insert(steps);
        }
    }
    let mut positions2 = HashSet::new();
    pos = (0, 0);
    let mut distances2 = HashMap::new();
    steps = 0;
    for (dir, n) in parsed[1].iter().copied() {
        for _ in 0..n {
            pos += dir;
            positions2.insert(pos);
            steps += 1;
            distances2.entry(pos).or_insert(steps);
        }
    }
    let inter = positions
        .intersection(&positions2)
        .map(|pos| distances[pos] + distances2[pos])
        .min()
        .unwrap();

    pv!(inter);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/03.txt");

    let parsed = input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|s| scanf!(s, "{}{}", Dir, isize).unwrap())
                .to_vec()
        })
        .to_vec();

    let mut positions = HashSet::new();
    let mut pos = (0isize, 0);
    for (dir, n) in parsed[0].iter().copied() {
        for _ in 0..n {
            pos += dir;
            positions.insert(pos);
        }
    }
    let mut positions2 = HashSet::new();
    pos = (0, 0);
    for (dir, n) in parsed[1].iter().copied() {
        for _ in 0..n {
            pos += dir;
            positions2.insert(pos);
        }
    }
    let inter = positions
        .intersection(&positions2)
        .map(|p| manhattan_i(*p, (0, 0)))
        .min()
        .unwrap();

    pv!(inter);
}
