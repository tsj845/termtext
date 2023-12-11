// use console::{Term, measure_text_width, Alignment, Key};
use console::{measure_text_width, Alignment};
use crossterm::event::read as read_input;
#[allow(unused_imports)]
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, ModifierKeyCode, MediaKeyCode, MouseEvent, MouseButton, MouseEventKind};
use std::io::{BufRead, Write, ErrorKind, Error};
use std::fs::{File, write};
use std::io;
use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};
use data::*;
// use chrono::{};

use crate::*;
use crate::reader::*;

static BOTTOM_TEXT: &str = "^X to quit, ^S to save, ^Q to force quit";

type BRes = io::Result<()>;
const BOK: BRes = Ok(());

const MINROWS: u64 = 10;
const MINCOLS: u64 = 50;

// enum InputAction {
//     NoAction,
//     QuitOk(String),
//     QuitErr(String),
//     Refresh,
//     Save,
//     DumpContent,
// }
// use InputAction::{NoAction,QuitOk,QuitErr,Refresh,Save,DumpContent};

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

impl Controller { // static testing functions
    pub fn arbtest() -> () {
        let x = LineList::from_iter(vec!["l1".to_owned(), "l2".to_owned()]);
        let laddr = Line::split_a(x.index(0), 1);
        // let laddr = line_to_ptr(Line::from_str("l0"));
        x.insert(laddr, 0);
        for l in x.clone().into_iter() {
            println!("{}", Line::to_string_a(l));
        }
    }
}

impl Controller { // small functions that change display config
    /// run after a crash happens where the terminal is not reset properly
    pub fn unfckterminal() -> () {
        unfckterminal();
    }
    pub fn pretend_size(&mut self, rows: u64, cols: u64) -> () {
        self.attrs.size = (rows, cols);
    }
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
        // x.terminal.begin()?;
        x.attrs.display.lastmod = x.meta.fmt_last_modified();
        x.activeline = x.list.index(0);
        if debugging(5) {x.test_readback();x.terminal.term.read_key()?;}
        if debugging(7) {
            x.terminal.clear_screen();
            println!("\x1b[38;2;200;200;0mWARNING:\x1b[0m DEBUG FLAG SEVEN IS SET, THIS WILL CAUSE ALL SAVE OPERATIONS TO FAIL SILENTLY\r");
            x.terminal.term.read_key()?;
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
            x.terminal.term.read_key()?;
        }
        // x.terminal.end()?;
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
        self.terminal.hide_cursor();
        self.terminal.queue();
        self.terminal.top_left();
        if self.cflag(DArea::TopText) {
            if self.cflag(DArea::TTSaved) {
                let lw = self.attrs.display.tt_left_len;
                if lw > 0 {
                    print!("{}", String::from_iter(std::iter::repeat(' ').take(lw)));
                    self.terminal.top_left();
                }
                let x = format!("{:?} {}\x1b[0m", self.attrs.size, match debugging(7){false=>"\x1b[38;2;0;200;0mSAVE ENABLED",_=>"\x1b[38;2;220;0;0mSAVE DISABLED"});
                print!("{}", &x);
                self.attrs.display.tt_left_len = measure_text_width(&x);
            }
            print!("{}", &console::pad_str(&(self.meta.title.clone()+"    "+&self.attrs.display.lastmod), self.attrs.size.1 as usize, Alignment::Center, None)[self.attrs.display.tt_left_len..]);
            println!("\x1b[D\x1b[38;2;150;150;150m|\x1b[0m\r")
            // println!("{}\r", &console::pad_str(&(self.meta.title.clone()+"    "+&self.attrs.display.lastmod), self.attrs.size.1 as usize, Alignment::Center, None)[self.attrs.display.tt_left_len..]);
        } else {
            self.terminal.down();
        }
        if self.cflag(DArea::EditArea) {
            // let msize = (self.attrs.size.0-2) * self.attrs.size.1;
            // let mut tsize: u64 = 0;
            // let mut ftext: String = String::new();
            let mut clineindex: u64 = 1; // set to one because the terminal.down() call before the EditArea check puts the cursor at row 1 col 0
            let clinemax: u64 = self.attrs.size.0 - 3; // -3 instead of -2 because the index is inclusive
            let maxwidth: usize = self.attrs.size.1 as usize; // cache so there aren't a lot of pointer derefs + readability
            for laddr in self.list.clone().into_iter() {
                if Line::get_linenum_a(laddr) < self.attrs.frame_start.1 {continue;} // go to start of lines that will be drawn
                // if tsize >= msize {break;}
                if clineindex > clinemax {break;}
                let fulstr: String = Line::to_string_a(laddr);
                if measure_text_width(&fulstr) > maxwidth {
                    print!("LINE TOO LONG");
                } else {
                    print!("{fulstr}");
                }
                self.terminal.clear_to_newline();
                print!("\r\n");
                clineindex += 1;
            }
        }
        if self.cflag(DArea::BotText) {
            self.terminal.set_cur_row(self.attrs.size.0);
            // print!("\x1b[{};1f", self.attrs.size.0);
            if self.cflag(DArea::BTCuP) {
                let lw = self.attrs.display.bt_left_len;
                if lw > 0 {
                    print!("{}", String::from_iter(std::iter::repeat(' ').take(lw)));
                    self.terminal.reset_col();
                    // self.terminal.set_cur_row(self.attrs.size.0);
                }
                let y = format!("{:?}", self.attrs.pos);
                // let y = format!("{:?} LKC: {}", self.attrs.pos, self._lastcode);
                print!("{}", &y);
                self.attrs.display.bt_left_len = y.len();
            }
            if self.cflag(DArea::BTAllE) {
                self.terminal.set_cur_pos(self.attrs.size.0, self.attrs.display.bt_left_len as u64);
                print!("{}", &console::pad_str(BOTTOM_TEXT, self.attrs.size.1 as usize, Alignment::Center, None)[self.attrs.display.bt_left_len..]);
            }
        }
        self.terminal.set_cur_pos(self.attrs.pos.0 + 1, self.attrs.pos.1);
        // self.terminal.flush();
        let _ = self.terminal.out.flush();
        self.terminal.show_cursor();
        return BOK;
    }
    fn end(&mut self) -> () {
        let _ = self.terminal.end();
        let _ = self.terminal.cleanup();
        self.save();
        self.meta.path.clear(); // ensure fs path is invalidated
        self.terminal.clear_screen();
        if !self.endreason.is_empty() {
            println!("QUIT FOR REASON: {}", self.endreason);
        }
    }
    fn input_loop(&mut self) -> BRes {
        self.terminal.begin()?;
        'outer: loop {
            let input: Event = read_input()?;
            match input {
                Event::Key(mut k) => {
                    if k.kind == KeyEventKind::Release {
                        continue 'outer;
                    }
                    if k.modifiers.contains(KeyModifiers::CONTROL) {
                        k.code = apply_key_ctrl(k.code)?;
                    } else if k.state.contains(KeyEventState::CAPS_LOCK) ^ k.modifiers.contains(KeyModifiers::SHIFT) {
                        k.code = apply_key_shift(k.code)?;
                    }
                    match (k.code, k.modifiers) {
                        (KeyCode::Char(c), _) => {
                            if c == 17 as char {self.waserr=true;self.endreason="FORCE QUIT".to_owned();break 'outer;} // ^Q
                            if c == 24 as char {self.endreason="CONTROL X".to_owned();break 'outer;} // ^X
                            if c == 19 as char {self.save();continue 'outer;} // ^S
                            if c == 20 as char { // ^T
                                if debugging(8) {
                                    self.__dumpcontent()?;
                                    self.terminal.clear_screen();
                                    self.aflag();
                                    self.render_screen()?;
                                }
                                continue 'outer;
                            }
                            if c == 18 as char { // ^R
                                self.aflag();
                                self.render_screen()?;
                                continue 'outer;
                            }
                            if c == 6 as char { // ^F
                                let _ = self.terminal.out.flush();
                                continue 'outer;
                            }
                            if c == 4 as char { // ^D
                                self.cflag(!0);
                                self.sflag(DArea::BTAllE | DArea::BotText);
                                self.attrs.display.bt_left_len = 0;
                                self.render_screen()?;
                                continue 'outer;
                                // self.terminal.save_raw();
                                // self.terminal.term.clear_screen()?;
                                // print!("POS: {:?}\n", self.attrs.pos);
                                // self.terminal.term.read_key()?;
                                // self.terminal.restore_raw();
                            }
                            self.sflag(DArea::BTCuP | DArea::BotText | DArea::EditArea | DArea::EACuL);
                            self.render_screen()?;
                        },
                        (KeyCode::Enter, _) => {
                            // let ac = line_to_ptr(Line::new());
                            let ac = Line::split_a(self.activeline, self.attrs.pos.1);
                            if self.attrs.pos.0 == 0 && self.attrs.pos.1 == 0 {
                                self.list.insert(ac, 0);
                                self.activeline = ac;
                            } else {
                                self.list.insert(ac, self.attrs.frame_start.0 + self.attrs.pos.0 + 1);
                            }
                            self._down()?;
                            self.sflag(DArea::BTCuP | DArea::BotText | DArea::EAAll);
                            self.render_screen()?;
                            // return Err(Error::new(ErrorKind::Other, concat!("DEBUGGING AT LINE ",line!())));
                        },
                        (KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right, _) => {
                            match k.code {
                                KeyCode::Up => {self._up()?;},
                                KeyCode::Down => {self._down()?;},
                                KeyCode::Left => {self._left()?;},
                                KeyCode::Right => {self._right()?;},
                                _ => {unreachable!();},
                                // _ => unsafe {std::hint::unreachable_unchecked();}
                            };
                            // return Err(Error::new(ErrorKind::Other, concat!("DEBUGGING AT LINE ",line!())));
                            self.sflag(DArea::BTCuP | DArea::BotText);
                            self.render_screen()?;
                        }
                        (_, _) => {},
                    };
                },
                _ => {},
            };
        }
        self.terminal.end()?;
        self.terminal.cleanup()?;
        Ok(())
    }
    // fn handle_input(&mut self) -> io::Result<InputAction> {
    //     let input: Input = self.terminal.read_input()?;
    //     match input {
    //         KeyIn(mut k) => {
    //             match k {
    //                 Key::Tab => {k=Key::Char('\t');},
    //                 Key::Enter => {k=Key::Char('\n');},
    //                 _ => {},
    //             };
    //             match k {
    //                 Key::Char(c) => {
    //                     let oreg = self.attrs.display.redisplay;
    //                     self.aflag();
    //                     if c == 0 as char {return Ok(QuitErr("NULL KEY".to_owned()));}
    //                     if c == 17 as char {return Ok(QuitErr("FORCE QUIT".to_owned()));}
    //                     if c == 19 as char {return Ok(Save);}
    //                     if c == 24 as char {return Ok(QuitOk("CONTROL X".to_owned()));}
    //                     if c == 20 as char {return Ok(DumpContent);}
    //                     if c == 18 as char {self.aflag();
    //                         return Ok(Refresh);
    //                         // return Err(Error::new(ErrorKind::InvalidInput, "CTRL-R"));
    //                     }
    //                     self.attrs.display.redisplay = oreg;
    //                     self._lastcode = c as u64;
    //                     self.sflag(DArea::EditArea);
    //                     self.sflag(DArea::BotText);
    //                     self.sflag(DArea::BTAll);
    //                     return Ok(Refresh);
    //                 },
    //                 _ => {
    //                     self.sflag(DArea::BotText);
    //                     self.sflag(DArea::BTCuP);
    //                     match k {
    //                         Key::ArrowUp => {self._lastcode=201;self._up()?},
    //                         Key::ArrowDown => {self._lastcode=202;self._down()?},
    //                         Key::ArrowLeft => {self._lastcode=203;self._left()?},
    //                         Key::ArrowRight => {self._lastcode=204;self._right()?},
    //                         Key::Del | Key::Backspace => {self.sflag(DArea::EditArea);self.sflag(DArea::BotText);self._delete()?},
    //                         Key::Alt | Key::Shift => {
    //                             if k == Key::Alt {
    //                                 self._lastcode = 257;
    //                             } else {
    //                                 self._lastcode = 258;
    //                             }
    //                             return Ok(Refresh);
    //                         },
    //                         _ => {self.cflag(DArea::BTCuP);self.cflag(DArea::BotText);}
    //                     };
    //                     self.sflag(DArea::BotText);
    //                     return Ok(Refresh);
    //                 }
    //             }
    //         },
    //         _ => {return Ok(NoAction);}
    //     };
    // }
    fn _init(&mut self) -> io::Result<()> {
        self.terminal.begin()?;
        self.terminal.clear_screen();
        self.aflag();
        self.render_screen()?;
        return BOK;
    }
    pub fn start(&mut self) -> BRes {
        // init
        let mut s: Position = self.terminal.size();
        if self.attrs.size.0 != 0 {
            s.0 = self.attrs.size.0;
        }
        if self.attrs.size.1 != 0 {
            s.1 = self.attrs.size.1;
        }
        if s.0 < MINROWS || s.1 < MINCOLS {
            self.waserr = true;
            self.endreason = "MINIMUM SIZE REQUIREMENTS NOT MET".to_string();
            self.end();
            return Ok(());
        }
        self.attrs.size = s;
        self.attrs.pref_x = 0;
        // self.attrs.pref_x = self.attrs.size.1;
        self._init()?;
        if debugging(6) {return BOK;}
        // run
        // loop {
        //     if self.endflag {
        //         self.end();
        //         break;
        //     }
        //     match self.handle_key()? {
        //         NoAction => {},
        //         QuitOk(x) => {self.endflag=true;self.endreason=x;},
        //         QuitErr(x) => {self.endflag=true;self.endreason=x;self.waserr=true;},
        //         Refresh => {self.render_screen()?;},
        //         Save => {self.save();},
        //         DumpContent => {
        //             if debugging(8) {
        //                 self.__dumpcontent()?;
        //                 self.render_screen()?;
        //             }
        //         },
        //     }
        // }
        // self.terminal.end()?;
        let r = catch_unwind(AssertUnwindSafe(|| self.input_loop()));
        match r {
            Ok(s) => {self.end();s},
            Err(e) => {let _ = self.terminal.cleanup();resume_unwind(e)},
        }
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
                    self.attrs.pos.1 = 0;
                    self.attrs.pref_x = 0;
                }
                else if self.attrs.mov_restrict.up || self.attrs.pos.0 > self.attrs.mov_restrict.up_max {
                    self.activeline = Line::get_prev_a(self.activeline);
                    let l2 = Line::len_a(self.activeline);
                    self.attrs.pos.1 = self.attrs.pref_x;
                    if self.attrs.pos.1 > l2 {
                        self.attrs.pos.1 = l2;
                    }
                    self.attrs.pos.0 -= 1;
                } else {b=false;}
            }
            if b {return BOK;}
        }
        self.attrs.pref_x = 0;
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
                    self.attrs.pos.1 = l;
                    self.attrs.pref_x = self.attrs.pos.1;
                }
                else if self.attrs.mov_restrict.down || self.attrs.pos.0 < self.attrs.mov_restrict.down_max {
                    self.activeline = Line::get_next_a(self.activeline);
                    let l2 = Line::len_a(self.activeline);
                    self.attrs.pos.1 = self.attrs.pref_x;
                    if self.attrs.pos.1 > l2 {
                        self.attrs.pos.1 = l2;
                    }
                    self.attrs.pos.0 += 1;
                } else {b=false;}
            }
            if b {return BOK;}
        }
        if !c1 {
            self.attrs.pref_x = l;
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
            self.attrs.pref_x = self.attrs.pos.1;
            self.attrs.suppress_move_errs = false;
            return r;
        }
        if self.attrs.mov_restrict.left || self.attrs.pos.1 > self.attrs.mov_restrict.left_max {self.attrs.pos.1-=1;
            self.attrs.pref_x = self.attrs.pos.1;
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
            self.attrs.pref_x = self.attrs.pos.1;
            self.attrs.suppress_move_errs = false;
            return r;
        }
        if self.attrs.mov_restrict.right || self.attrs.pos.1 < self.attrs.mov_restrict.right_max || true {self.attrs.pos.1+=1;
            self.attrs.pref_x = self.attrs.pos.1;
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
        self.attrs.display.msg = msg.to_owned();
        self.sflag(DArea::BTMsg);
        return BOK;
    }
    fn __dumpcontent(&mut self) -> BRes {
        self.terminal.clear_screen();
        self.test_readback();
        self.terminal.save_raw();
        self.terminal.term.read_key()?;
        self.terminal.restore_raw();
        return BOK;
    }
    fn gflag(&self, x: u64) -> bool {
        gflag(self.attrs.display.redisplay, x)
    }
    fn sflag(&mut self, x: u64) -> () {
        sflag(&mut self.attrs.display.redisplay, x);
    }
    fn cflag(&mut self, x: u64) -> bool {
        cflag(&mut self.attrs.display.redisplay, x)
    }
    fn aflag(&mut self) -> () {
        self.attrs.display.redisplay = !0;
    }
}