use std::io::BufRead;
use std::fs::File;

use crate::*;

pub struct FileMeta {
    pub title: String,
}

pub struct Controller {
    pub list: LineList,
    pub meta: FileMeta,
}

impl Controller {
    pub fn from_file(title: String, path: String) -> Self {
        let x = Self {
            list: LineList::from_iter(std::io::BufReader::new(File::open(path).unwrap()).lines()),
            meta: FileMeta { title },
        };
        if debugging(5) {x.test_readback();}
        return x;
    }
    fn test_readback(&self) -> () {
        println!("TESTING");
        for addr in self.list.clone().into_iter() {
            println!("READBACK ({:x}, {})", addr, 0);
            println!("{}", Line::to_string_a(addr));
        }
        if debugging(4) {println!("LOOP EXECUTED");}
    }
}