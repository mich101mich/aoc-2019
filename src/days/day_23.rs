use crate::utils::*;

use std::sync::*;

pub fn run() {
	#[allow(unused_variables)]
	let input = include_str!("../input/23.txt");

	let mut rec = vec![];
	let mut send = vec![];

	for _ in 0..50 {
		let (s, r) = mpsc::channel();
		rec.push(r);
		send.push(s);
	}

	let nat = mpsc::channel();

	let mut handles = vec![];

	let idles = Arc::new(atomic::AtomicUsize::new(0));

	for i in 0..50 {
		let mut bot = IntProgram::with_default(input, -1);
		bot.inputs.push(i);
		let send = send.clone();
		let rec = rec.remove(0);
		let nat = nat.0.clone();
		let idles = idles.clone();

		let handle = std::thread::spawn(move || {
			let mut idle = false;
			loop {
				if let Ok((x, y)) = rec.try_recv() {
					bot.inputs.push(x);
					bot.inputs.push(y);
				}
				let addr = match step_int_code(&mut bot, true) {
					IntResult::Finished => return,
					IntResult::Output(o) => o,
					IntResult::Step => continue,
					IntResult::Default => {
						if !idle {
							idles.fetch_add(1, atomic::Ordering::Relaxed);
							idle = true;
						}
						continue;
					}
				};
				if idle {
					idles.fetch_sub(1, atomic::Ordering::Relaxed);
					idle = false;
				}

				let x = int_code(&mut bot, true).unwrap();
				let y = int_code(&mut bot, true).unwrap();
				if addr == 255 {
					nat.send((x, y)).unwrap();
					continue;
				}
				if let Some(target) = send.get(addr as usize) {
					target.send((x, y)).unwrap();
				}
			}
		});
		handles.push(handle);
	}
	handles.push(std::thread::spawn(move || {
		let mut last = 2_251_799;
		loop {
			let idle = idles.load(atomic::Ordering::Relaxed);
			pv!(idle);
			if idle == 50 {
				let mut packet = nat.1.recv().unwrap();
				while let Ok(p) = nat.1.try_recv() {
					packet = p;
				}
				pv!(packet);
				if last == packet.1 {
					pv!(packet.1); // 19406
					std::process::exit(0);
				}
				last = packet.1;
				send[0].send(packet).unwrap();
			}
			std::thread::sleep(std::time::Duration::from_millis(1000));
		}
	}));
	for handle in handles {
		handle.join().unwrap();
	}
}

#[allow(unused)]
pub fn part_one() {
	#[allow(unused_variables)]
	let input = include_str!("../input/23.txt");

	let mut rec = vec![];
	let mut send = vec![];

	for _ in 0..50 {
		let (s, r) = std::sync::mpsc::channel();
		rec.push(r);
		send.push(s);
	}

	let mut handles = vec![];

	for i in 0..50 {
		let mut bot = IntProgram::with_default(input, -1);
		bot.inputs.push(i);
		let mut send = send.clone();
		let rec = rec.remove(0);

		let handle = std::thread::spawn(move || loop {
			if let Ok((x, y)) = rec.try_recv() {
				bot.inputs.push(x);
				bot.inputs.push(y);
			}
			let addr = match step_int_code(&mut bot, true) {
				IntResult::Finished => return,
				IntResult::Output(o) => o,
				_ => continue,
			};

			let x = int_code(&mut bot, true).unwrap();
			let y = int_code(&mut bot, true).unwrap();
			if addr == 255 {
				pv!(y);
				std::process::exit(0);
			}
			if let Some(target) = send.get_mut(addr as usize) {
				target.send((x, y)).unwrap();
			}
		});
		handles.push(handle);
	}
	for handle in handles {
		handle.join().unwrap();
	}
}
