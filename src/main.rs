use std::process::Command;
use std::io;
use std::fs::OpenOptions;
//use std::io::prelude::*;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::env;

fn read_line()-> String {
	io::stdout().flush().unwrap();
    let mut s = String::new();
    let _ = io::stdin().read_line(&mut s);
	s
}

fn cmd_parse(cmd: String) -> Vec<String> {
    let s = cmd.split_whitespace().map(|e|e.parse().unwrap())
        .collect::<Vec<String>>();
    s
}

fn cmd_run(cmd: &str,args: &[String]) -> Command {
    let mut cmd_r = Command::new(cmd);
	cmd_r.args(args);
    cmd_r
}

fn cmd_cd(paths:&str) -> bool {
	let path = Path::new(paths);

	if path.is_dir() == false {
		println!("{} Not a directory.",paths);
		return false
	}else{
		match env::set_current_dir(path.canonicalize().unwrap().as_path()) {
			Ok(_c) => true,
			Err(_) => false,
		}
	}
}

fn main() {

	let file = match OpenOptions::new()
            .read(true)
            .write(true)
			.append(true)
            .create(true).open(".rush_history") {
		Ok(file) => file,
		Err(e) => {
					println!("An error occurred while open opening file .rush_history:{}",e);
					return;
				}
	};

	let mut history = BufWriter::new(file);

	loop {
		print!("rush$ ");
		let command = read_line();

		if command == "\n" {
			print!("");
			continue;
		}

		history.write(command.as_bytes()).unwrap();
		history.flush().unwrap();

		let args = cmd_parse(command);

		if args[0] == "quit" {
			println!("godbye!");
			break;
		}else if args[0] == "cd" {
			cmd_cd(&args[1]);
		}else {

			let mut cmd_param= cmd_run(&args[0],&args[1..]);

    		let cmd_result = match cmd_param.output() {
				Ok(c) => c,
				Err(_) => {
					println!("failed execute command!");
					continue;
				}
			};

			let cmd = match cmd_result.status.success() {
				true => cmd_result.stdout,
				false => cmd_result.stderr,
			};

    		println!("{}",String::from_utf8(cmd).unwrap());
		}
	}
}
