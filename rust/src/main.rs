extern crate better_panic;
extern crate crossterm;

use rust::*;
use std::env;

fn main() {
    better_panic::Settings::new().verbosity(better_panic::Verbosity::Medium).install();
    if env::args().collect::<Vec<String>>().contains(&("unfck".to_owned())) {
        Controller::unfckterminal();
        return;
    }
    let mut control: Controller = Controller::from_file("testfile.txt".to_owned(), "testfile.txt".to_owned()).unwrap();
    // control.pretend_size(0, 50);
    match control.start() {
        Ok(_) => {},
        Err(e) => {let _ = crossterm::terminal::disable_raw_mode();println!("\x1b[2J\x1b[f");Err::<(),std::io::Error>(e).unwrap();},
    };
}