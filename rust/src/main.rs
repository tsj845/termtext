extern crate better_panic;
extern crate crossterm;

use rust::*;
use std::{env, process};
// use std::fs;

fn main() {
    better_panic::Settings::new().verbosity(better_panic::Verbosity::Medium).install();
    let args = env::args().collect::<Vec<String>>();
    let execpath = args[0].clone();
    if args.contains(&("arbtest".to_owned())) {
        Controller::arbtest();
        return;
    }
    if args.contains(&("unfck".to_owned())) {
        Controller::unfckterminal();
        return;
    }
    if args.contains(&("-real".to_owned())) {
        let mut control: Controller = Controller::from_file("testfile.txt".to_owned(), "testfile.txt".to_owned()).unwrap();
        // control.pretend_size(0, 50);
        match control.start() {
            Ok(_) => {},
            Err(e) => {let _ = crossterm::terminal::disable_raw_mode();println!("\x1b[2J\x1b[f");Err::<(),std::io::Error>(e).unwrap();},
        };
        return;
    }
    let _ = process::Command::new(std::env::current_dir().unwrap().join(execpath)).arg("-real").args(&args[1..])
    // .stderr(fs::OpenOptions::new().write(true).truncate(true).open("dbgout.txt").unwrap())
    .spawn().unwrap().wait();
    print!("\n\r");
    Controller::unfckterminal();
}