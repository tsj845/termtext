use console::{Term, measure_text_width, Alignment, Key};
use std::io::{BufRead, Write};
use std::fs::{File, write};
use std::io;
use std::time::SystemTime;
// use chrono::{};

use crate::*;

static BOTTOM_TEXT: &str = "^X to quit, ^S to save, ^Q to force quit";

type BRes = io::Result<()>;
enum InputAction {
    NoAction,
    QuitOk,
    QuitErr(String),
    Refresh,
    Save,
}
use InputAction::{NoAction,QuitOk,QuitErr,Refresh,Save};

pub struct FileMeta {
    pub title: String,
    pub path: String,
    pub histpath: String,
    pub last_modified: String,
}

#[derive(Clone, Copy)]
struct _MoveRestrict {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub up_max: u64,
    pub down_max: u64,
    pub left_max: u64,
    pub right_max: u64,
}impl _MoveRestrict {fn new()->Self{Self { up: false, down: true, left: false, right: false, up_max: 2, down_max: 0, left_max: 0, right_max: 0 }}}

#[derive(Clone, Copy)]
struct Attrs {
    pub size: (u64, u64),
    pub frame_start: (u64, u64),
    pub pos: (u64, u64),
    pub mov_restrict: _MoveRestrict,
}

pub struct Controller {
    pub list: LineList,
    pub meta: FileMeta,
    endflag: bool,
    endreason: String,
    terminal: Term,
    attrs: Attrs,
}

impl Controller {
    pub fn from_file(title: String, path: String) -> Self {
        let x = Self {
            list: LineList::from_iter(std::io::BufReader::new(File::open(&path).unwrap()).lines()),
            meta: FileMeta { title, path: path.clone(), histpath: "".to_owned(), last_modified: format!("{:?}", SystemTime::now().duration_since(std::fs::metadata(&path).unwrap().modified().unwrap()).unwrap().as_secs()) },
            endflag: false,
            endreason: String::new(),
            terminal: Term::stdout(),
            attrs: Attrs { size: (0,0), frame_start: (0,0), pos: (0,0), mov_restrict: _MoveRestrict::new() }
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
        let msize = (self.attrs.size.0-2) * self.attrs.size.1;
        let mut tsize: u64 = 0;
        let mut ftext: String = String::new();
        for laddr in self.list.clone().into_iter() {
            if Line::get_linenum_a(laddr) < self.attrs.frame_start.1 {continue;} // go to start of screen
            if tsize >= msize {break;}
            let l = Line::len_a(laddr);
            if (tsize + l) >= msize {
                ftext.push_str(&Line::substr_a(laddr, 0, msize-tsize));
                // tsize = msize;
                ftext.push('\n');
                break;
            } else {
                ftext.push_str(&Line::to_string_a(laddr));
                tsize += l;
                ftext.push('\n');
            }
        }
        // if ftext.as_bytes()[(tsize-1) as usize] == '\n' as u8 {
        //     ftext.pop();
        // }
        self.terminal.clear_screen()?;
        let x = format!("{:?}", self.attrs.size);
        print!("{}", &x);
        println!("{}", console::pad_str(&(self.meta.title.clone()+"    "+&self.meta.last_modified), self.attrs.size.1 as usize - x.len(), Alignment::Center, None));
        self.terminal.write_all(ftext.as_bytes())?;
        self.terminal.write_all(console::pad_str(BOTTOM_TEXT, self.attrs.size.1 as usize, Alignment::Center, None).as_bytes())?;
        self.terminal.write_all(&[13,10])?;
        return Ok(());
    }
    fn end(&mut self) -> () {
        self.save();
        self.meta.path.clear(); // ensure fs path is invalidated
        self.terminal.clear_screen().unwrap();
        if !self.endreason.is_empty() {
            println!("QUIT FOR REASON: {}", self.endreason);
        }
    }
    fn handle_input(&mut self) -> io::Result<InputAction> {
        let k: Key = self.terminal.read_key()?;
        match k {
            Key::Char(c) => {
                if c == 17 as char {return Ok(QuitErr("FORCE QUIT".to_owned()));}
                if c == 19 as char {return Ok(Save);}
                if c == 24 as char {return Ok(QuitErr("CONTROL X".to_owned()));}
                return Ok(NoAction);
            },
            _ => {
                match k {
                    Key::ArrowUp => {self._up()?},
                    Key::ArrowDown => {self._down()?},
                    Key::ArrowLeft => {self._left()?},
                    Key::ArrowRight => {self._right()?},
                    _ => {}
                };
                return Ok(NoAction);
            }
        }
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
            self.attrs.size =  (x.0 as u64, x.1 as u64);
        }
        self._init().unwrap();
        if debugging(6) {return;}
        // run
        loop {
            if self.endflag {
                self.end();
                break;
            }
            match self.handle_input().unwrap() {
                NoAction => {},
                QuitOk => {self.endflag=true;},
                QuitErr(x) => {self.endflag=true;self.endreason=x;},
                Refresh => {self.render_screen().unwrap();},
                Save => {self.save();},
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

impl Controller {
    fn _up(&mut self) -> BRes {
        if self.attrs.mov_restrict.up || self.attrs.pos.0 > self.attrs.mov_restrict.up_max {return self.terminal.move_cursor_up(1);}
        Ok(())
    }
    fn _down(&mut self) -> BRes {
        if self.attrs.mov_restrict.down {return self.terminal.move_cursor_down(1);}
        Ok(())
    }
    fn _left(&mut self) -> BRes {
        if self.attrs.mov_restrict.left {return self.terminal.move_cursor_left(1);}
        Ok(())
    }
    fn _right(&mut self) -> BRes {
        if self.attrs.mov_restrict.right {return self.terminal.move_cursor_right(1);}
        Ok(())
    }
}