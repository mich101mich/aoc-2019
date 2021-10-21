use crate::utils::*;

use std::io::{stdin, Read};

const PRINT: bool = false;

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/25.txt");
    let mut code = IntProgram::new(input, vec![]);

    let mut text = String::new();
    let mut backup = vec![];
    let mut blacklist = HashSet::new();
    let mut last_item = None;
    let mut rng = rand::thread_rng();
    let mut remaining = HashMap::new();
    let mut dirs = vec![];
    let mut room_name = String::new();

    blacklist.insert(String::from("infinite loop"));
    blacklist.insert(String::from("giant electromagnet"));

    loop {
        backup.push(code.clone());
        let failed = execute(&mut code, &mut text);

        if failed {
            backup.pop();
            code = backup.pop().unwrap();
            blacklist.insert(last_item.take().unwrap());
            if PRINT {
                println!("========== LOADING BACKUP ==========");
            }
            continue;
        }

        if last_item.is_none() {
            room_name = text
                .lines()
                .find_map(|l| l.strip_prefix("== "))
                .unwrap_or("")
                .to_string();

            dirs = text
                .lines()
                .skip_while(|l| *l != "Doors here lead:")
                .take_while(|l| !l.is_empty())
                .filter_map(|l| l.strip_prefix("- "))
                .map(|s| s.to_string())
                .to_vec();
        } else {
            last_item = None;
        }

        let items = text
            .lines()
            .skip_while(|l| *l != "Items here:")
            .take_while(|l| !l.is_empty())
            .filter_map(|l| l.strip_prefix("- "))
            .map(|s| s.to_string())
            .to_vec();

        let remain_dirs = remaining
            .entry(room_name.to_string())
            .or_insert_with(|| dirs.iter().map(|d| d.to_string()).to_vec());

        let command;
        if let Some(item) = items
            .iter()
            .map(|i| i.to_string())
            .find(|i| !blacklist.contains(i))
        {
            command = format!("take {}", item);
            last_item = Some(item);
        } else if room_name == "Security Checkpoint ==" {
            remain_dirs.clear();
            if remaining.values().all(|v| v.is_empty()) {
                command = String::from("inv");
            } else {
                command = String::from("east");
            }
        } else if !remain_dirs.is_empty() {
            command = remain_dirs.remove(rng.gen_range(0..remain_dirs.len()));
        } else {
            command = dirs.choose(&mut rng).unwrap().to_string();
        }

        // let mut action = String::new();
        // stdin().read_line(&mut action);
        // code.add_input(&action);

        if PRINT {
            println!("{}", command);
        }
        code.add_input(&command);
        code.inputs.push(b'\n' as isize);
        if command == "inv" {
            break;
        }
    }

    execute(&mut code, &mut text);

    let items = text
        .lines()
        .skip_while(|l| *l != "Items in your inventory:")
        .filter_map(|l| l.strip_prefix("- "))
        .map(|s| s.to_string())
        .to_vec();

    try_items(&mut code, &items, &mut text);
}

fn try_items(code: &mut IntProgram, items: &[String], text: &mut String) -> bool {
    if let Some((item, rest)) = items.split_first() {
        if PRINT {
            println!("with {}", item);
        }
        if try_items(code, rest, text) {
            return true;
        }

        code.add_input(&format!("drop {}\n", item));
        if PRINT {
            println!("without {}", item);
        }
        if try_items(code, rest, text) {
            return true;
        }
        code.add_input(&format!("take {}\n", item));
        false
    } else {
        code.add_input("south\n");
        let res = execute(code, text);
        if res {
            println!(
                "{}",
                text.lines()
                    .last()
                    .unwrap()
                    .split_ascii_whitespace()
                    .find_map(|w| w.parse::<usize>().ok())
                    .unwrap()
            );
        } else if PRINT {
            println!("No");
        }
        res
    }
}

fn execute(code: &mut IntProgram, text: &mut String) -> bool {
    text.clear();
    loop {
        let c = match step_int_code(code, true) {
            IntResult::Finished => return true,
            IntResult::Output(o) => o as u8 as char,
            IntResult::Step => continue,
            IntResult::MissingInput => return false,
        };
        text.push(c);
        if PRINT {
            print!("{}", c);
        }
    }
}
