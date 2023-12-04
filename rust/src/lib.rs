extern crate console;
// extern crate chrono;
extern crate crossterm;
extern crate once_cell;
extern crate lazy_static;
mod adaptor;
mod statics;
use statics::*;
use line::*;
mod data;
pub mod line;
pub mod controller;

pub use line::*;
pub use controller::Controller;