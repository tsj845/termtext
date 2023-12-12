extern crate console;
extern crate crossterm;
mod reader;
mod statics;
use statics::*;
mod data;
pub mod line;
pub mod controller;

pub use line::*;
pub use controller::Controller;