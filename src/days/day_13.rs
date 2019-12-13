use crate::utils::*;

use piston_window::*;

pub fn run() {
	#[allow(unused_variables)]
	let input = include_str!("../input/13.txt");
	let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [42 * 20, 24 * 20])
		.exit_on_esc(true)
		.build()
		.unwrap();

	let colors = [
		[0.0, 0.0, 0.0, 1.0],
		[0.5, 0.5, 0.5, 1.0],
		[1.0, 0.0, 0.0, 1.0],
		[0.0, 1.0, 0.0, 1.0],
		[0.0, 0.0, 1.0, 1.0],
	];

	let inputs = [
		0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1,
		1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1,
		-1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1, -1, -1, -1, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
	];
	let stop = 4840;
	let mut states = vec![];

	let mut game = vec![vec![0; 42]; 24];
	let mut code = IntProgram::new(input, vec![]);
	code.mem[0] = 2;
	for v in inputs.iter() {
		code.inputs.push(*v);
	}
	let mut finished_draw = false;
	let mut score = 0;
	while !finished_draw || score < stop {
		if let Some(x) = int_code(&mut code, true) {
			let y = int_code(&mut code, true).unwrap();
			let id = int_code(&mut code, true).unwrap();
			if x < 0 {
				score = id;
				println!("{}", id);
			} else {
				game[y as usize][x as usize] = id;
				if y as usize == game.len() - 1 && x as usize == game[0].len() - 1 {
					finished_draw = true;
				}
			}
		} else {
			return;
		}
	}

	let mut frame = 0;
	while let Some(event) = window.next() {
		window.draw_2d(&event, |context, graphics, _device| {
			clear([1.0; 4], graphics);
			for (y, row) in game.iter().enumerate() {
				for (x, v) in row.iter().enumerate() {
					rectangle(
						colors[*v as usize],
						[x as f64 * 20.0, y as f64 * 20.0, 20.0, 20.0],
						context.transform,
						graphics,
					);
				}
			}
		});
		if let Event::Input(
			Input::Button(ButtonArgs {
				state: ButtonState::Press,
				button: Button::Keyboard(key),
				..
			}),
			_,
		) = event
		{
			match key {
				keyboard::Key::A => {
					states.push((code.clone(), game.clone()));
					code.inputs.push(-1)
				}
				keyboard::Key::D => {
					states.push((code.clone(), game.clone()));
					code.inputs.push(1)
				}
				keyboard::Key::Space => {
					states.push((code.clone(), game.clone()));
					code.inputs.push(0)
				}
				keyboard::Key::U => {
					let state = states.pop().unwrap();
					code = state.0;
					game = state.1;
				}
				_ => {}
			}
		}
		frame += 1;
		if (score < stop || frame > 2) && !code.inputs.is_empty() {
			frame = 0;
		} else {
			continue;
		}
		while let Some(x) = int_code(&mut code, true) {
			let y = int_code(&mut code, true).unwrap();
			let id = int_code(&mut code, true).unwrap();
			if x < 0 {
				score = id;
				println!("{}", id);
			} else {
				game[y as usize][x as usize] = id;
				if id == 4 {
					break;
				}
			}
		}
	}
}

#[allow(unused)]
pub fn part_one() {
	#[allow(unused_variables)]
	let input = include_str!("../input/13.txt");
	let mut game = vec![vec![0; 50]; 30];
	let mut code = IntProgram::new(input, vec![]);
	while let Some(x) = int_code(&mut code, true) {
		let y = int_code(&mut code, true).unwrap();
		let id = int_code(&mut code, true).unwrap();
		game[y as usize][x as usize] = id;
	}
	for row in game.iter() {
		for v in row {
			if *v != 0 {
				print!("{}", v);
			} else {
				print!(" ");
			}
		}
		println!();
	}
	let cnt = game
		.iter()
		.flat_map(|row| row.iter())
		.filter(|v| **v == 2)
		.count();
	pv!(cnt);
}
