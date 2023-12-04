extern crate better_panic;
extern crate crossterm;

use rust::*;

fn main() {
    better_panic::Settings::new().verbosity(better_panic::Verbosity::Medium).install();
    let mut control: Controller = Controller::from_file("testfile.txt".to_owned(), "testfile.txt".to_owned()).unwrap();
    match control.start() {
        Ok(_) => {},
        Err(e) => {let _ = crossterm::terminal::disable_raw_mode();println!("\x1b[2J\x1b[f");Err::<(),std::io::Error>(e).unwrap();},
    };
}