use console::{Term, TermFeatures, TermTarget, measure_text_width, Alignment, Key};
use std::io::{BufRead, Read, Write};
use std::fs::{File, write};

use crate::*;

pub struct FileMeta {
    pub title: String,
    pub path: String,
    pub histpath: String,
}

pub struct Controller {
    pub list: LineList,
    pub meta: FileMeta,
    errflag: bool,
    terminal: Term,
}

impl Controller {
    pub fn from_file(title: String, path: String) -> Self {
        let x = Self {
            list: LineList::from_iter(std::io::BufReader::new(File::open(&path).unwrap()).lines()),
            meta: FileMeta { title, path, histpath: "".to_owned() },
            errflag: false,
            terminal: Term::stdout(),
        };
        if debugging(5) {x.test_readback();}
        return x;
    }
    /// saves modified file content, EXTREMELY SLOW
    fn save(&mut self) -> () {
        let mut sbuf: String;
        {
            let x = self.list.total_size as usize;
            sbuf = String::with_capacity(match x < 100 {true=>100,_=>x});
        }
        for laddr in self.list.clone().into_iter() {
            sbuf.push_str(&String::from_utf8(Line::iter_over(laddr).into_iter().collect::<Vec<u8>>()).unwrap());
            sbuf.push('\n');
        }
        sbuf.pop();
        write(&self.meta.path, sbuf).unwrap();
    }
    fn end(&mut self) -> () {
        self.save();
        self.meta.path.clear(); // ensure fs path is invalidated
        self.terminal.clear_screen();
    }
    pub fn start(&mut self) -> () {
        loop {
            if self.errflag {
                self.end();
                break;
            }
        }
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