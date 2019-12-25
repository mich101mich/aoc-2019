use crate::utils::*;
use std::io::Read;
pub fn run() {
	#[allow(unused_variables)]
	let input = include_str!("../input/25.txt");
	let mut code = IntProgram::new(input, vec![]);
	// let mut past = vec![];

	let mut actions = vec![
		"south",
		"east",
		"take space heater",
		"west",
		"west",
		"take shell",
		"east",
		"north",
		"west",
		"north",
		"take jam",
		"east",
		"south",
		"take asterisk",
		"south",
		"take klein bottle",
		"east",
		"take spool of cat6",
		"west",
		"north",
		"north",
		"west",
		"north",
		"take astronaut ice cream",
		"north",
		"east",
		"south",
		"take space law space brochure",
		"north",
		"west",
		"south",
		"south",
		"south",
		"south",
		"west",
		"south",
	];

	let mut inv = vec![
		"spool of cat6",
		"space law space brochure",
		"asterisk",
		"jam",
		"shell",
		"astronaut ice cream",
		"space heater",
		"klein bottle",
	];
	let mut ground = vec![];

	let mut rng = thread_rng();

	loop {
		let mut text = String::new();

		while let Some(c) = int_code(&mut code, true) {
			text.push(c as u8 as char);
			print!("{}", c as u8 as char);
			if text.ends_with("Command?") {
				break;
			}
		}
		if !actions.is_empty() {
			let action = actions.remove(0);
			println!("{}", action);
			for c in action.chars() {
				code.inputs.push(c as u8 as i64);
			}
			code.inputs.push(b'\n' as i64);
			continue;
		}

		if !text.contains("Alert!") {
			pv!(inv);
			return;
		}

		if !inv.is_empty() && (ground.is_empty() || rng.gen()) {
			for c in "drop ".chars() {
				code.inputs.push(c as u8 as i64);
			}
			let item = *inv.choose(&mut rng).expect("inv");
			inv.retain(|i| *i != item);
			ground.push(item);
			for c in item.chars() {
				code.inputs.push(c as u8 as i64);
			}
			code.inputs.push(b'\n' as i64);
		} else {
			for c in "take ".chars() {
				code.inputs.push(c as u8 as i64);
			}
			let item = *ground.choose(&mut rng).expect("ground");
			ground.retain(|i| *i != item);
			inv.push(item);
			for c in item.chars() {
				code.inputs.push(c as u8 as i64);
			}
			code.inputs.push(b'\n' as i64);
		}
		for c in "south\n".chars() {
			code.inputs.push(c as u8 as i64);
		}
		text.clear();
		while let Some(c) = int_code(&mut code, true) {
			text.push(c as u8 as char);
			print!("{}", c as u8 as char);
			if text.ends_with("Command?") {
				break;
			}
		}

		// past.push(code.clone());

		// let mut action = String::new();
		// std::io::stdin().read_line(&mut action);

		// if action.starts_with("undo") {
		// 	code = past.pop().unwrap();
		// 	action.clear();
		// 	std::io::stdin().read_line(&mut action);
		// }

		// for c in action.chars() {
		// 	code.inputs.push(c as u8 as i64);
		// }
	}

	//pv!(parsed);
}
