use crate::utils::*;

use std::sync::{
    atomic::{AtomicU8, Ordering::SeqCst},
    mpsc, Arc,
};

#[allow(unused)]
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
    let wake_nat = mpsc::channel();

    let mut handles = vec![];

    let idles = Arc::new(AtomicU8::new(0));

    for i in 0..50 {
        let mut bot = IntProgram::new(input, vec![]);
        bot.inputs.push(i);
        let send = send.clone();
        let rec = rec.remove(0);
        let nat = nat.0.clone();
        let wake_nat = wake_nat.0.clone();
        let idles = idles.clone();
        let mut idle_count = 0;

        let handle = std::thread::spawn(move || loop {
            let addr = match step_int_code(&mut bot, true) {
                IntResult::Finished => return,
                IntResult::Output(o) => o,
                IntResult::Step => continue,
                IntResult::MissingInput => {
                    let mut msg = rec.try_recv().ok();
                    if idle_count >= 10 {
                        msg = msg.or_else(|| rec.recv().ok());
                    }
                    if let Some((x, y)) = msg {
                        bot.inputs.push(x);
                        bot.inputs.push(y);
                        if idle_count > 0 {
                            idles.fetch_sub(1, SeqCst);
                            idle_count = 0;
                        }
                    } else {
                        bot.inputs.push(-1);
                        if idle_count == 0 {
                            let prev_cnt = idles.fetch_add(1, SeqCst);
                            if prev_cnt == 49 {
                                wake_nat.send(());
                            }
                        }
                        idle_count += 1;
                    }
                    continue;
                }
            };

            let x = int_code(&mut bot, true).unwrap();
            let y = int_code(&mut bot, true).unwrap();
            if addr == 255 {
                nat.send((x, y)).unwrap();
                continue;
            } else {
                send[addr as usize].send((x, y)).unwrap();
            }
        });
        handles.push(handle);
    }
    handles.push(std::thread::spawn(move || {
        let mut last = 2_251_799;
        loop {
            let _ = wake_nat.1.recv().unwrap();
            let idle = idles.load(SeqCst);
            if idle != 50 {
                continue;
            }
            if let Some(packet) = nat.1.try_iter().last() {
                if last == packet.1 {
                    pv!(packet.1);
                    std::process::exit(0);
                }
                last = packet.1;
                send[0].send(packet).unwrap();
            }
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
        let mut bot = IntProgram::new(input, vec![]);
        bot.inputs.push(i);
        let mut send = send.clone();
        let rec = rec.remove(0);
        let mut idle_count = 0;

        let handle = std::thread::spawn(move || loop {
            let addr = match step_int_code(&mut bot, true) {
                IntResult::Finished => return,
                IntResult::Output(o) => o,
                IntResult::Step => continue,
                IntResult::MissingInput => {
                    let mut msg = rec.try_recv().ok();
                    if idle_count >= 10 {
                        msg = msg.or_else(|| rec.recv().ok());
                    }
                    if let Some((x, y)) = msg {
                        bot.inputs.push(x);
                        bot.inputs.push(y);
                        idle_count = 0;
                    } else {
                        bot.inputs.push(-1);
                        idle_count += 1;
                    }
                    continue;
                }
            };

            let x = int_code(&mut bot, true).unwrap();
            let y = int_code(&mut bot, true).unwrap();
            if addr == 255 {
                pv!(y);
                std::process::exit(0);
            } else {
                send[addr as usize].send((x, y)).unwrap();
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
