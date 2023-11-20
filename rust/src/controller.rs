use console::{Term, TermFeatures, TermTarget, measure_text_width, Alignment, Key};
use std::io::{BufRead, Read, Write};
use std::fs::{File, write};
use std::io;

use crate::*;

pub struct FileMeta {
    pub title: String,
    pub path: String,
    pub histpath: String,
}

struct Attrs {
    pub size: (u64, u64),
    pub frame_start: (u64, u64),
}

pub struct Controller {
    pub list: LineList,
    pub meta: FileMeta,
    errflag: bool,
    terminal: Term,
    cache: Attrs,
}

impl Controller {
    pub fn from_file(title: String, path: String) -> Self {
        let x = Self {
            list: LineList::from_iter(std::io::BufReader::new(File::open(&path).unwrap()).lines()),
            meta: FileMeta { title, path, histpath: "".to_owned() },
            errflag: false,
            terminal: Term::stdout(),
            cache: Attrs { size: (0,0), frame_start: (0,0) }
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
    fn render_screen(&mut self) -> io::Result<()> {
        let msize = self.cache.size.0 * self.cache.size.1;
        let mut tsize: u64 = 0;
        let mut ftext: String = String::new();
        for laddr in self.list.clone().into_iter() {
            if Line::get_linenum_a(laddr) < self.cache.frame_start.1 {continue;} // go to start of screen
            if tsize >= msize {break;}
            let l = Line::len_a(laddr);
            if (tsize + l) >= msize {
                ftext.push_str(&Line::substr_a(laddr, 0, msize-tsize));
                tsize = msize;
                break;
            } else {
                ftext.push_str(&Line::to_string_a(laddr));
                tsize += l;
                ftext.push('\n');
            }
        }
        if ftext.as_bytes()[(tsize-1) as usize] == '\n' as u8 {
            ftext.pop();
        }
        self.terminal.clear_screen()?;
        self.terminal.write_all(ftext.as_bytes())?;
        return Ok(());
    }
    fn end(&mut self) -> () {
        self.save();
        self.meta.path.clear(); // ensure fs path is invalidated
        self.terminal.clear_screen().unwrap();
    }
    fn handle_input(&mut self) -> io::Result<()> {
        return Ok(());
    }
    fn _init(&mut self) -> io::Result<()> {
        self.terminal.clear_screen()?;
        self.render_screen()?;
        return Ok(());
    }
    pub fn start(&mut self) -> () {
        // init
        {
            let x = self.terminal.size();
            self.cache.size =  (x.0 as u64, x.1 as u64);
        }
        self._init().unwrap();
        println!("{:?}", self.cache.size);
        if debugging(6) {return;}
        // run
        loop {
            if self.errflag {
                self.end();
                break;
            }
            self.handle_input().unwrap();
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