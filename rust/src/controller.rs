// use console::{Term, measure_text_width, Alignment, Key};
use console::{measure_text_width, Alignment};
use std::io::{BufRead, Write, ErrorKind, Error};
use std::fs::{File, write};
use std::io;
use data::*;
// use chrono::{};

use crate::*;
use crate::adaptor::*;

static BOTTOM_TEXT: &str = "^X to quit, ^S to save, ^Q to force quit";

type BRes = io::Result<()>;
const BOK: BRes = Ok(());

enum InputAction {
    NoAction,
    QuitOk(String),
    QuitErr(String),
    Refresh,
    Save,
    DumpContent,
}
use InputAction::{NoAction,QuitOk,QuitErr,Refresh,Save,DumpContent};

pub struct Controller {
    pub list: LineList,
    pub meta: FileMeta,
    activeline: u64,
    endflag: bool,
    waserr: bool,
    endreason: String,
    terminal: Term,
    attrs: Attrs,
    _lastcode: u64,
}

impl Controller {
    pub fn from_file(title: String, path: String) -> io::Result<Self> {
        let mut x = Self {
            list: LineList::from_iter(std::io::BufReader::new(File::open(&path).unwrap()).lines()),
            meta: FileMeta { title, path: path.clone(), histpath: "".to_owned(), last_modified: std::fs::metadata(&path).unwrap().modified().unwrap() },
            activeline: 0,
            endflag: false,
            waserr: false,
            endreason: String::new(),
            terminal: Term::new(),
            attrs: Attrs { size: (0,0), frame_start: (0,0), pos: (0,0), pref_x: 0, mov_restrict: _MoveRestrict::new(), suppress_move_errs: false, display: _Display::new() },
            _lastcode: 0,
        };
        x.terminal.begin()?;
        x.attrs.display.lastmod = x.meta.fmt_last_modified();
        x.activeline = x.list.index(0);
        if debugging(5) {x.test_readback();x.terminal.read_key()?;}
        if debugging(7) {
            x.terminal.clear_screen();
            println!("\x1b[38;2;200;200;0mWARNING:\x1b[0m DEBUG FLAG SEVEN IS SET, THIS WILL CAUSE ALL SAVE OPERATIONS TO FAIL SILENTLY\r");
            x.terminal.read_key()?;
        }
        if debugging(63) {
            x.terminal.clear_line();
            // println!("{}", x.meta.fmt_last_modified());
            // aflag(&mut x.attrs.display.redisplay);
            // println!("{:#X}", x.attrs.display.redisplay);
            // x.attrs.display.redisplay = 0;
            x.endflag = true;
            x.endreason = "FLAG 63".to_owned();
            x.waserr = true;
            x.terminal.read_key()?;
        }
        x.terminal.end()?;
        return Ok(x);
    }
    /// saves modified file content, EXTREMELY SLOW
    fn save(&mut self) -> () {
        if debugging(7) {return;}
        if self.waserr {return;} // if an error occurred, it's likely that either data was corrupted, or a line's contents became invalid
        self.attrs.display.lastmod = self.meta.fmt_last_modified();
        let mut sbuf: String;
        {
            let x = self.list.total_size as usize;
            sbuf = String::with_capacity(match x < 100 {true=>100,_=>x});
        }
        for laddr in self.list.clone().into_iter() {
            sbuf.push_str(&String::from_utf8(Line::iter_over(laddr).into_iter().collect::<Vec<u8>>()).unwrap());
            sbuf.push('\r');
            sbuf.push('\n');
        }
        sbuf.pop();
        write(&self.meta.path, sbuf).unwrap();
    }
    fn render_screen(&mut self) -> io::Result<()> {
        if self.attrs.display.redisplay == 0 { // don't do anything if nothing needs to be redrawn
            return BOK;
        }
        let msize = (self.attrs.size.0-2) * self.attrs.size.1;
        let mut tsize: u64 = 0;
        let mut ftext: String = String::new();
        if self.gflag(DArea::EditArea) {
            for laddr in self.list.clone().into_iter() {
                if Line::get_linenum_a(laddr) < self.attrs.frame_start.1 {continue;} // go to start of screen
                if tsize >= msize {break;}
                let l = Line::len_a(laddr);
                if (tsize + l) >= msize {
                    ftext.push_str(&Line::substr_a(laddr, 0, msize-tsize));
                    // tsize = msize;
                    ftext.push('\r');
                    ftext.push('\n');
                    break;
                } else {
                    ftext.push_str(&Line::to_string_a(laddr));
                    tsize += l;
                    ftext.push('\r');
                    ftext.push('\n');
                }
            }
            // if ftext.as_bytes()[(tsize-1) as usize] == '\n' as u8 {
            //     ftext.pop();
            // }
            ftext.pop();
        }
        // Term::clear_screen();
        if self.cflag(DArea::TopText) {
            // print!("\x1b[T\x1b[1;1f");
            print!("\x1b[1;1f");
            if self.cflag(DArea::TTSaved) {
                let lw = self.attrs.display.top_text_left_length;
                if lw > 0 {
                    print!("{}", String::from_iter(std::iter::repeat(' ').take(lw)));
                    print!("\x1b[1;1f");
                }
                let x = format!("{:?} {}\x1b[0m", self.attrs.size, match debugging(7){false=>"\x1b[38;2;0;200;0mSAVE ENABLED",_=>"\x1b[38;2;220;0;0mSAVE DISABLED"});
                print!("{}", &x);
                self.attrs.display.top_text_left_length = measure_text_width(&x);
            }
            println!("{}\r", &console::pad_str(&(self.meta.title.clone()+"    "+&self.attrs.display.lastmod), self.attrs.size.1 as usize, Alignment::Center, None)[self.attrs.display.top_text_left_length..]);
        } else {
            print!("\x1b[2;1f");
        }
        if self.cflag(DArea::EditArea) {
            print!("\x1b[1T\x1b[1B\x1b[0J\x1b[1S\x1b[1A");
            print!("{ftext}");
        }
        self.sflag(DArea::BotText);
        self.sflag(DArea::BTAll);
        if self.cflag(DArea::BotText) {
            print!("\x1b[{};1f", self.attrs.size.0);
            if self.cflag(DArea::BTCuP) {
                let lw = self.attrs.display.bot_text_left_length;
                if lw > 0 {
                    print!("{}", String::from_iter(std::iter::repeat(' ').take(lw)));
                    print!("\x1b[{};1f", self.attrs.size.0);
                }
                let y = format!("{:?} LKC: {}", self.attrs.pos, self._lastcode);
                print!("{}", &y);
                self.attrs.display.bot_text_left_length = y.len();
            }
            if self.cflag(DArea::BTAll) {
                print!("\x1b[{};{}f", self.attrs.size.0, self.attrs.display.bot_text_left_length + 1);
                print!("{}", &console::pad_str(BOTTOM_TEXT, self.attrs.size.1 as usize, Alignment::Center, None)[self.attrs.display.bot_text_left_length..]);
            }
        }
        // self.terminal.move_cursor_to(0, self.attrs.size.0 as usize)?;
        // self.terminal.write_all(&[13,10])?;
        if debugging(9) {
            print!("\x1b[f{:?}", self.attrs.pos);
            self.terminal.read_key()?;
        }
        Term::set_cur_pos(self.attrs.pos.1, self.attrs.pos.0 + 1);
        return BOK;
    }
    fn end(&mut self) -> () {
        let _ = self.terminal.end();
        self.save();
        self.meta.path.clear(); // ensure fs path is invalidated
        self.terminal.clear_screen();
        if !self.endreason.is_empty() {
            println!("QUIT FOR REASON: {}", self.endreason);
        }
    }
    fn handle_input(&mut self) -> io::Result<InputAction> {
        let input: Input = self.terminal.read_input()?;
        match input {
            KeyIn(mut k) => {
                match k {
                    Key::Tab => {k=Key::Char('\t');},
                    Key::Enter => {k=Key::Char('\n');},
                    _ => {},
                };
                match k {
                    Key::Char(c) => {
                        let oreg = self.attrs.display.redisplay;
                        self.aflag();
                        if c == 0 as char {return Ok(QuitErr("NULL KEY".to_owned()));}
                        if c == 17 as char {return Ok(QuitErr("FORCE QUIT".to_owned()));}
                        if c == 19 as char {return Ok(Save);}
                        if c == 24 as char {return Ok(QuitOk("CONTROL X".to_owned()));}
                        if c == 20 as char {return Ok(DumpContent);}
                        self.attrs.display.redisplay = oreg;
                        self._lastcode = c as u64;
                        self.sflag(DArea::EditArea);
                        self.sflag(DArea::BotText);
                        self.sflag(DArea::BTAll);
                        return Ok(Refresh);
                    },
                    _ => {
                        self.sflag(DArea::BotText);
                        self.sflag(DArea::BTCuP);
                        match k {
                            Key::ArrowUp => {self._up()?},
                            Key::ArrowDown => {self._down()?},
                            Key::ArrowLeft => {self._left()?},
                            Key::ArrowRight => {self._right()?},
                            Key::Del | Key::Backspace => {self.sflag(DArea::EditArea);self.sflag(DArea::BotText);self._delete()?},
                            Key::Alt | Key::Shift => {
                                if k == Key::Alt {
                                    self._lastcode = 257;
                                } else {
                                    self._lastcode = 258;
                                }
                                return Ok(Refresh);
                            },
                            _ => {self.cflag(DArea::BTCuP);self.cflag(DArea::BotText);}
                        };
                        self.sflag(DArea::BotText);
                        return Ok(Refresh);
                    }
                }
            },
            _ => {return Ok(NoAction);}
        };
    }
    fn _init(&mut self) -> io::Result<()> {
        self.terminal.begin()?;
        self.terminal.clear_screen();
        self.aflag();
        self.render_screen()?;
        return BOK;
    }
    pub fn start(&mut self) -> BRes {
        // init
        self.attrs.size = self.terminal.size();
        self.attrs.pref_x = 0;
        // self.attrs.pref_x = self.attrs.size.1;
        self._init()?;
        if debugging(6) {return BOK;}
        // run
        loop {
            if self.endflag {
                self.end();
                break;
            }
            match self.handle_input()? {
                NoAction => {},
                QuitOk(x) => {self.endflag=true;self.endreason=x;},
                QuitErr(x) => {self.endflag=true;self.endreason=x;self.waserr=true;},
                Refresh => {self.render_screen()?;},
                Save => {self.save();},
                DumpContent => {
                    if debugging(8) {
                        self.__dumpcontent()?;
                        self.render_screen()?;
                    }
                },
            }
        }
        self.terminal.end()?;
        BOK
    }
    fn test_readback(&mut self) -> () {
        self.terminal.save_raw();
        println!("TESTING");
        for addr in self.list.clone().into_iter() {
            println!("READBACK ({:x}, {})", addr, Line::len_a(addr));
            println!("READBACK N/P ({:x}, {:x})", Line::get_prev_a(addr), Line::get_next_a(addr));
            println!("{}", Line::to_string_a(addr));
        }
        if debugging(4) {println!("LOOP EXECUTED");}
        self.terminal.restore_raw();
    }
}

macro_rules! nocur {
    {$s:ident,$($code:tt)+} => {
        $s.terminal.hide_cursor();
        $(
            $code
        )+
        $s.terminal.show_cursor();
    };
}

impl Controller {
    fn _up(&mut self) -> BRes {
        if self.attrs.pos.0 != 0 || self.attrs.pos.1 > 0 {
            let mut b = true;
            nocur!{self,
                if self.attrs.pos.0 == 0 {
                    // let x = self.attrs.pos.1;
                    self.attrs.pos.1 = 0;
                    // Term::left();
                }
                else if self.attrs.mov_restrict.up || self.attrs.pos.0 > self.attrs.mov_restrict.up_max {
                    self.activeline = Line::get_prev_a(self.activeline);
                    let l2 = Line::len_a(self.activeline);
                    self.attrs.pos.1 = self.attrs.pref_x;
                    if self.attrs.pos.1 > l2 {
                        self.attrs.pos.1 = l2;
                    }
                    self.attrs.pos.0 -= 1;
                    // Term::up();
                } else {b=false;}
            }
            if b {return BOK;}
        }
        if !self.attrs.suppress_move_errs{return BOK;}
        return Err(Error::from(ErrorKind::ConnectionReset));
    }
    fn _down(&mut self) -> BRes {
        let c1 = self.attrs.pos.0 < self.list.size()-1;
        let l = Line::len_a(self.activeline);
        if self.attrs.pos.0 < (self.attrs.size.0-4) && (c1 || self.attrs.pos.1 < l) {
            let mut b = true;
            nocur!{self,
                if !c1 {
                    // Term::right_n(l-self.attrs.pos.1);
                    self.attrs.pos.1 = l;
                }
                else if self.attrs.mov_restrict.down || self.attrs.pos.0 < self.attrs.mov_restrict.down_max {
                    self.activeline = Line::get_next_a(self.activeline);
                    let l2 = Line::len_a(self.activeline);
                    self.attrs.pos.1 = self.attrs.pref_x;
                    if self.attrs.pos.1 > l2 {
                        self.attrs.pos.1 = l2;
                    }
                    self.attrs.pos.0 += 1;
                    // Term::down();
                } else {b=false;}
            }
            if b {return BOK;}
        }
        if !self.attrs.suppress_move_errs{return BOK;}
        return Err(Error::from(ErrorKind::ConnectionReset));
    }
    fn _left(&mut self) -> BRes {
        if self.attrs.pos.1 == 0 {
            self.attrs.suppress_move_errs = true;
            let mut r: BRes = BOK;
            nocur!{self,
                match self._up() {
                    Ok(_) => {
                        self.attrs.pos.1 = Line::len_a(self.activeline);
                    },
                    Err(x) => {
                        if x.kind() != ErrorKind::ConnectionReset {r = Err(x);}
                    }
                };
            }
            self.attrs.suppress_move_errs = false;
            return r;
        }
        if self.attrs.mov_restrict.left || self.attrs.pos.1 > self.attrs.mov_restrict.left_max {self.attrs.pos.1-=1;
            // Term::left();
        }
        BOK
    }
    fn _right(&mut self) -> BRes {
        if self.attrs.pos.1 >= Line::len_a(self.activeline) {
            self.attrs.suppress_move_errs = true;
            let mut r: BRes = BOK;
            nocur!{self,
                match self._down() {
                    Ok(_) => {
                        self.attrs.pos.1 = 0;
                    },
                    Err(x) => {
                        if x.kind() != ErrorKind::ConnectionReset {r = Err(x);}
                    }
                };
            }
            self.attrs.suppress_move_errs = false;
            return r;
        }
        if self.attrs.mov_restrict.right || self.attrs.pos.1 < self.attrs.mov_restrict.right_max || true {self.attrs.pos.1+=1;
            // Term::right();
        }
        BOK
    }
    fn _delete(&mut self) -> BRes {
        if self.attrs.pos.1 == 0 {
            return self._msg("deleting whitespace isn't implemented yet!");
        }
        Line::remove_char_a(self.activeline, self.attrs.pos.1-1);
        self.attrs.pos.1 -= 1;
        self.list.total_size -= 1;
        BOK
    }
}

impl Controller {
    /// todo
    fn _msg(&mut self, msg: &str) -> BRes {
        return BOK;
    }
    fn __dumpcontent(&mut self) -> BRes {
        self.terminal.clear_screen();
        self.test_readback();
        self.terminal.read_key()?;
        return BOK;
    }
    fn gflag(&self, x:DArea) -> bool {
        gflag(self.attrs.display.redisplay, x as u64)
    }
    fn sflag(&mut self, x:DArea) -> () {
        self.attrs.display.redisplay |= DArea::allflags(x);
        sflag(&mut self.attrs.display.redisplay, x as u64);
    }
    fn cflag(&mut self, x:DArea) -> bool {
        cflag(&mut self.attrs.display.redisplay, x as u64)
    }
    fn aflag(&mut self) -> () {
        aflag(&mut self.attrs.display.redisplay);
    }
}