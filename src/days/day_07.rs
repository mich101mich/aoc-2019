use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/07.txt");

    let code = IntProgram::new(input, vec![]);

    let mut programs = vec![code; 5];

    let mut max = 0;
    for ids in permutohedron::Heap::new(&mut (5..10).to_vec()) {
        let mut programs = programs.clone();
        let mut value = 0;
        for (id, code) in ids.iter().zip(programs.iter_mut()) {
            code.inputs.push(*id);
            code.inputs.push(value);
            if let Some(v) = int_code(code, true) {
                value = v;
            } else {
                panic!("end");
            }
        }
        let mut current = 0;
        let new_value = 'outer: loop {
            for code in programs.iter_mut() {
                code.inputs.push(value);
                if let Some(v) = int_code(code, true) {
                    value = v;
                } else {
                    break 'outer current;
                }
            }
            current = value;
        };
        max = max.max(new_value);
    }
    pv!(max);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/07.txt");

    let code = IntProgram::new(input, vec![]);

    let mut programs = vec![code; 5];

    let mut max = 0;
    for ids in permutohedron::Heap::new(&mut (0..5).to_vec()) {
        let mut programs = programs.clone();
        let mut value = 0;
        for (id, code) in ids.iter().zip(programs.iter_mut()) {
            code.inputs.push(*id);
            code.inputs.push(value);
            if let Some(v) = int_code(code, true) {
                value = v;
            } else {
                panic!("end");
            }
        }
        max = max.max(value);
    }
    pv!(max);
}
