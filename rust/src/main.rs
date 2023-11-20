extern crate better_panic;

use rust::*;

fn main() {
    better_panic::Settings::new().verbosity(better_panic::Verbosity::Medium).install();
    let control: Controller = Controller::from_file("test".to_owned(), "testfile.txt".to_owned());
}