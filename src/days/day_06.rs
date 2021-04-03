use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/06.txt");

    let parsed = input.lines().map(|l| l.split(')').to_vec()).to_vec();

    let mut children = HashMap::new();
    let mut parent = HashMap::new();
    for x in parsed {
        children.entry(x[0]).or_insert_with(Vec::new).push(x[1]);
        parent.insert(x[1], x[0]);
    }

    let path = dijkstra_search(
        |p| {
            children
                .get(p)
                .map(|v| v.iter())
                .unwrap_or_else(|| [].iter())
                .copied()
                .chain(parent.get(p).copied())
        },
        |_, _| 1,
        |_| true,
        "YOU",
        &["SAN"],
    );
    pv!(path["SAN"].cost - 2);
}

#[allow(unused)]
fn c(parent: &HashMap<&str, Vec<&str>>, current: &str) -> usize {
    let child = match parent.get(current) {
        Some(b) => b,
        None => return 0,
    };
    let mut cn = child.len();
    for chil in child.iter() {
        cn += c(parent, chil);
    }
    cn
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/06.txt");

    let parsed = input.lines().map(|l| l.split(')').to_vec()).to_vec();

    let mut all = HashSet::new();
    let mut children = HashMap::new();
    let mut parent = HashMap::new();
    for x in parsed {
        children.entry(x[0]).or_insert_with(Vec::new).push(x[1]);
        parent.insert(x[1], x[0]);
        all.insert(x[0]);
        all.insert(x[1]);
    }

    let root = all.iter().find(|p| !parent.contains_key(*p)).unwrap();

    let mut remaining = VecDeque::new();
    remaining.push_back((root, 0));
    let mut count = 0;
    while let Some((p, orbits)) = remaining.pop_front() {
        count += orbits;

        if let Some(children) = children.get(p) {
            for c in children {
                remaining.push_back((c, orbits + 1));
            }
        }
    }

    pv!(count);
}
