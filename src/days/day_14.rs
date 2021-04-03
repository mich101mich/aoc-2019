use crate::utils::*;

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/14.txt");
    // let input = ;

    fn parse_chem(ch: &str) -> (u64, String) {
        scanf!(ch, "{} {}", u64, String)
    }

    let parsed = input
        .lines()
        .map(|l| {
            let mut sp = l.split(" => ");
            let input = sp.next().unwrap();
            let output = sp.next().unwrap();
            (parse_chem(output), input.split(", ").map(parse_chem).to_vec())
        })
        //.map(parse)
        //.map(|l| scanf!(l, "{}", isize))
        .to_vec()
        //.sum::<isize>()
        ;
    let mut map = HashMap::new();
    for (output, input) in parsed.iter() {
        map.insert(output.1.clone(), (input, output.0));
    }

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

    let mut lower = 1;
    let mut upper = 100_000_000u64;

    while upper - lower > 1 {
        let mut predecessors = predecessors.clone();

        let mid = (upper + lower) / 2;

        let mut required = HashMap::new();
        required.insert("FUEL", mid);

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
            let factor = (amount as f32 / *outcome as f32).ceil() as u64;
            for (i_amount, i_name) in inputs.iter() {
                *required.entry(i_name).or_insert(0) += i_amount * factor;
            }
        }
        let amount = required["ORE"];
        if amount > 1_000_000_000_000u64 {
            upper = mid;
        } else {
            lower = mid;
        }
    }
    pv!(lower);
    pv!(upper);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/14.txt");
    // let input = ;

    fn parse_chem(ch: &str) -> (usize, String) {
        scanf!(ch, "{} {}", usize, String)
    }

    let parsed = input
        .lines()
        .map(|l| {
            let mut sp = l.split(" => ");
            let input = sp.next().unwrap();
            let output = sp.next().unwrap();
            (parse_chem(output), input.split(", ").map(parse_chem).to_vec())
        })
        //.map(parse)
        //.map(|l| scanf!(l, "{}", isize))
        .to_vec()
        //.sum::<isize>()
        ;
    let mut map = HashMap::new();
    for (output, input) in parsed.iter() {
        map.insert(output.1.clone(), (input, output.0));
    }

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
    pv!(predecessors);
    pv!(required);
}
