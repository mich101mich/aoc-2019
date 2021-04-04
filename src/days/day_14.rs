use crate::utils::*;

fn parse_chem(ch: &str) -> (usize, String) {
    scanf!(ch, "{} {}", usize, String).unwrap()
}

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/14.txt");

    let map = input
        .lines()
        .map(|l| {
            let (input, n, output) = scanf!(l, "{} => {} {}", String, usize, String).unwrap();
            (output, (input.split(", ").map(parse_chem).to_vec(), n))
        })
        .to_map();

    let mut predecessors = HashMap::new();
    for name in map.keys() {
        predecessors.insert(name.as_str(), vec![]);
    }
    predecessors.insert("ORE", vec![]);

    for (parent, (inputs, _)) in map.iter() {
        for (_, child) in inputs.iter() {
            predecessors.get_mut(child.as_str()).unwrap().push(parent);
        }
    }

    let mut required = HashMap::new();
    required.insert("FUEL", 1.0);

    while let Some((&next, _)) = predecessors.iter().find(|(_, v)| v.is_empty()) {
        if next == "ORE" {
            break;
        }
        predecessors.remove(next);
        for other in predecessors.values_mut() {
            other.retain(|v| *v != next);
        }
        let amount = required.remove(next).unwrap();
        let (inputs, outcome) = &map[next];
        let factor = amount / *outcome as f64;
        for (i_amount, i_name) in inputs.iter() {
            *required.entry(i_name).or_insert(0.0) += *i_amount as f64 * factor;
        }
    }
    let amount = f64::floor(1_000_000_000_000.0 / required["ORE"]) as usize;
    pv!(amount);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/14.txt");

    let map = input
        .lines()
        .map(|l| {
            let (input, n, output) = scanf!(l, "{} => {} {}", String, usize, String).unwrap();
            (output, (input.split(", ").map(parse_chem).to_vec(), n))
        })
        .to_map();

    let mut predecessors = HashMap::new();
    for name in map.keys() {
        predecessors.insert(name.as_str(), vec![]);
    }
    predecessors.insert("ORE", vec![]);

    for (parent, (inputs, _)) in map.iter() {
        for (_, child) in inputs.iter() {
            predecessors.get_mut(child.as_str()).unwrap().push(parent);
        }
    }

    let mut required = HashMap::new();
    required.insert("FUEL", 1);

    while let Some((&next, _)) = predecessors.iter().find(|(_, v)| v.is_empty()) {
        if next == "ORE" {
            break;
        }
        predecessors.remove(next);
        for other in predecessors.values_mut() {
            other.retain(|v| *v != next);
        }
        let amount = required.remove(next).unwrap();
        let (inputs, outcome) = &map[next];
        let factor = (amount as f32 / *outcome as f32).ceil() as usize;
        for (i_amount, i_name) in inputs.iter() {
            *required.entry(i_name).or_insert(0) += i_amount * factor;
        }
    }
    pv!(required);
}
