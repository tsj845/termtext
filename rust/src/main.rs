extern crate better_panic;
extern crate console;

use rust::*;

fn main() {
    better_panic::Settings::new().verbosity(better_panic::Verbosity::Medium).install();
    {
        Line::new();
        Line::new();
    }
    let control: Controller = Controller::from_file("test".to_owned(), "testfile.txt".to_owned());
}