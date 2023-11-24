use console::{Term, measure_text_width, Alignment, Key};
use std::io::{BufRead, Write, ErrorKind, Error};
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
    pub last_modified: u64,
} impl FileMeta {
    fn fmt_last_modified(&self) -> String {
        let years: u64 = self.last_modified / 31536000;
        let rweeks = self.last_modified % 31536000;
        let weeks: u64 = rweeks / 640800;
        let rdays = rweeks % 640800;
        let days: u64 = rdays / 86400;
        let rhours = rdays % 86400;
        let hours: u64 = rhours / 3600;
        let rminutes = rhours % 3600;
        let minutes: u64 = rhours / 60;
        let seconds: u64 = rminutes % 60;
        return format!("{}y-{}w-{}d-{}h-{}m-{}s", years, weeks, days, hours, minutes, seconds);
    }
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
}impl _MoveRestrict {fn new()->Self{Self { up: true, down: true, left: true, right: true, up_max: 0, down_max: 0, left_max: 0, right_max: 0 }}}

#[derive(Clone, Copy)]
struct Attrs {
    pub size: (u64, u64),
    pub frame_start: (u64, u64),
    pub pos: (u64, u64),
    pub mov_restrict: _MoveRestrict,
    pub suppress_move_errs: bool,
}

pub struct Controller {
    pub list: LineList,
    pub meta: FileMeta,
    activeline: u64,
    endflag: bool,
    endreason: String,
    terminal: Term,
    attrs: Attrs,
}

impl Controller {
    pub fn from_file(title: String, path: String) -> Self {
        let mut x = Self {
            list: LineList::from_iter(std::io::BufReader::new(File::open(&path).unwrap()).lines()),
            meta: FileMeta { title, path: path.clone(), histpath: "".to_owned(), last_modified: SystemTime::now().duration_since(std::fs::metadata(&path).unwrap().modified().unwrap()).unwrap().as_secs() },
            activeline: 0,
            endflag: false,
            endreason: String::new(),
            terminal: Term::stdout(),
            attrs: Attrs { size: (0,0), frame_start: (0,0), pos: (0,0), mov_restrict: _MoveRestrict::new(), suppress_move_errs: false }
        };
        x.activeline = x.list.index(0);
        if debugging(5) {x.test_readback();x.terminal.read_key().unwrap();}
        if debugging(7) {
            x.terminal.clear_screen().unwrap();
            println!("\x1b[38;2;200;200;0mWARNING:\x1b[0m DEBUG FLAG SEVEN IS SET, THIS WILL CAUSE ALL SAVE OPERATIONS TO FAIL SILENTLY");
            x.terminal.read_key().unwrap();
        }
        return x;
    }
    /// saves modified file content, EXTREMELY SLOW
    fn save(&mut self) -> () {
        if debugging(7) {return;}
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
        ftext.pop();
        self.terminal.clear_screen()?;
        print!("\x1b[{};{}f", self.attrs.size.0-1, 0);
        let y = format!("{:?}", self.attrs.pos);
        print!("{}", &y);
        print!("{}", &console::pad_str(BOTTOM_TEXT, self.attrs.size.1 as usize, Alignment::Center, None)[y.len()..]);
        print!("\x1b[T\x1b[1;1f");
        let x = format!("{:?} {}\x1b[0m", self.attrs.size, match debugging(7){false=>"\x1b[38;2;0;200;0mSAVE ENABLED",_=>"\x1b[38;2;220;0;0mSAVE DISABLED"});
        print!("{}", &x);
        println!("{}", &console::pad_str(&(self.meta.title.clone()+"    "+&self.meta.fmt_last_modified()), self.attrs.size.1 as usize, Alignment::Center, None)[measure_text_width(&x)..]);
        self.terminal.write_all(ftext.as_bytes())?;
        // self.terminal.move_cursor_to(0, self.attrs.size.0 as usize)?;
        // self.terminal.write_all(&[13,10])?;
        self.terminal.move_cursor_to(self.attrs.pos.1 as usize, self.attrs.pos.0 as usize + 1)?;
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
                return Ok(Refresh);
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
            println!("READBACK ({:x}, {})", addr, Line::len_a(addr));
            println!("READBACK N/P ({:x}, {:x})", Line::get_prev_a(addr), Line::get_next_a(addr));
            println!("{}", Line::to_string_a(addr));
        }
        if debugging(4) {println!("LOOP EXECUTED");}
    }
}

impl Controller {
    fn _up(&mut self) -> BRes {
        if self.attrs.pos.0 != 0 {
            if self.attrs.mov_restrict.up || self.attrs.pos.0 > self.attrs.mov_restrict.up_max {
                self.activeline = Line::get_prev_a(self.activeline);
                self.attrs.pos.0 -= 1;
                return self.terminal.move_cursor_up(1);
            }
        }
        if !self.attrs.suppress_move_errs{return Ok(());}
        return Err(Error::from(ErrorKind::ConnectionReset));
    }
    fn _down(&mut self) -> BRes {
        if !(self.attrs.pos.0 >= (self.attrs.size.0-4) || self.attrs.pos.0 >= (self.list.size()-1)) {
            if self.attrs.mov_restrict.down || self.attrs.pos.0 < self.attrs.mov_restrict.down_max {
                self.activeline = Line::get_next_a(self.activeline);
                self.attrs.pos.0 += 1;
                return self.terminal.move_cursor_down(1);
            }
        }
        if !self.attrs.suppress_move_errs{return Ok(());}
        return Err(Error::from(ErrorKind::ConnectionReset));
    }
    fn _left(&mut self) -> BRes {
        if self.attrs.pos.1 == 0 {
            self.attrs.suppress_move_errs = true;
            let mut r: BRes = Ok(());
            match self._up() {
                Ok(_) => {
                    self.attrs.pos.1 = Line::len_a(self.activeline)-1;
                },
                Err(x) => {
                    if x.kind() != ErrorKind::ConnectionReset {r = Err(x);}
                }
            };
            self.attrs.suppress_move_errs = false;
            return r;
        }
        if self.attrs.mov_restrict.left || self.attrs.pos.1 > self.attrs.mov_restrict.left_max {self.attrs.pos.1-=1;return self.terminal.move_cursor_left(1);}
        Ok(())
    }
    fn _right(&mut self) -> BRes {
        if self.attrs.pos.1 >= (Line::len_a(self.activeline)-1) {
            self.attrs.suppress_move_errs = true;
            let mut r: BRes = Ok(());
            match self._down() {
                Ok(_) => {
                    self.attrs.pos.1 = 0;
                },
                Err(x) => {
                    if x.kind() != ErrorKind::ConnectionReset {r = Err(x);}
                }
            };
            self.attrs.suppress_move_errs = false;
            return r;
        }
        if self.attrs.mov_restrict.right || self.attrs.pos.1 < self.attrs.mov_restrict.right_max || true {self.attrs.pos.1+=1;return self.terminal.move_cursor_right(1);}
        Ok(())
    }
}