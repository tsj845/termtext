use std::io::{Error, ErrorKind};
use std::io::{Stdout, stdout, Write};

use crossterm::cursor::*;
use crossterm::event::*;
use crossterm::terminal::*;
use crossterm::{queue, execute};
// use once_cell::unsync::Lazy;
// use lazy_static::lazy_static;
use std::io;

pub use crossterm::cursor::position;
pub use crossterm::event::KeyCode;
pub use console::Key;

use console::Term as Terminal;

// pub enum ModKey {
//     None,
//     Alt,
//     Ctrl,
//     Command,
//     Shift,
// }

// pub enum Key {
//     Unkown,
//     Char(char, ModKey),
//     Func(u8, ModKey),
//     Up(ModKey),
//     Down(ModKey),
//     Left(ModKey),
//     Right(ModKey),
//     Delete(ModKey),
// }

pub enum Input {
    KeyIn(Key),
    Ignore,
}

pub use Input::{KeyIn, Ignore};

/// row, column
pub type Position = (u64, u64);

pub struct Term {
    out: Stdout,
    term: Terminal,
    _raw: bool,
    _oraw: bool,
}

impl Term {
    pub fn new() -> Self {
        let mut x = Self {out:stdout(),term:Terminal::stdout(),_raw:false,_oraw:false};
        execute!(x.out, PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES)).unwrap();
        execute!(x.out, PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES)).unwrap();
        execute!(x.out, PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_ALTERNATE_KEYS)).unwrap();
        return x;
    }
    pub fn save_raw(&mut self) -> () {self._oraw=self._raw;self._raw=false;disable_raw_mode().unwrap();}
    pub fn restore_raw(&mut self) -> () {if self._oraw != self._raw {enable_raw_mode().unwrap();}self._raw=self._oraw;}
    pub fn begin(&mut self) -> io::Result<()> {
        self._raw = true;
        enable_raw_mode()
    }
    pub fn end(&mut self) -> io::Result<()> {
        self._raw = false;
        disable_raw_mode()
    }
    #[cfg_attr(do_inline, inline(always))]
    // pub fn set_cur_pos(p: Position) -> () {print!("\x1b[{};{}H", p.1, p.0);}
    pub fn set_cur_pos(x: u64, y: u64) -> () {print!("\x1b[{};{}f", y+1, x+1);}
    #[cfg_attr(do_inline, inline(always))]
    pub fn up() -> () {print!("\x1b[A");}
    #[cfg_attr(do_inline, inline(always))]
    pub fn up_n(n: u64) -> () {print!("\x1b[{}A", n);}
    #[cfg_attr(do_inline, inline(always))]
    pub fn down() -> () {print!("\x1b[B");}
    #[cfg_attr(do_inline, inline(always))]
    pub fn down_n(n: u64) -> () {print!("\x1b[{}B", n);}
    #[cfg_attr(do_inline, inline(always))]
    pub fn right() -> () {print!("\x1b[C");}
    #[cfg_attr(do_inline, inline(always))]
    pub fn right_n(n: u64) -> () {print!("\x1b[{}C", n);}
    #[cfg_attr(do_inline, inline(always))]
    pub fn left() -> () {print!("\x1b[D");}
    #[cfg_attr(do_inline, inline(always))]
    pub fn left_n(n: u64) -> () {print!("\x1b[{}D", n);}
    pub fn hide_cursor(&mut self) -> () {self.term.hide_cursor().unwrap();}
    pub fn show_cursor(&mut self) -> () {self.term.show_cursor().unwrap();}
    pub fn size(&mut self) -> Position {
        let s = self.term.size_checked().unwrap();
        (s.0 as u64, s.1 as u64)
    }
    #[cfg_attr(do_inline, inline(always))]
    pub fn clear_screen(&mut self) -> () {execute!(self.out, Clear(ClearType::All)).unwrap();print!("\x1b[f");}
    #[cfg_attr(do_inline, inline(always))]
    pub fn clear_line(&mut self) -> () {execute!(self.out, Clear(ClearType::CurrentLine)).unwrap();print!("\r");}
    pub fn read_input(&mut self) -> io::Result<Input> {
        if !self._raw {
            return Err(Error::new(ErrorKind::NotConnected, "RAW MODE NOT ENABLED"));
        }
        // match || -> io::Result<Input> {
        match read()? {
            Event::Key(mut x) => {
                if x.kind == KeyEventKind::Release {
                    return Ok(Ignore);
                }
                if x.modifiers.contains(KeyModifiers::SHIFT) {
                    match x.code {
                        KeyCode::Char(c) => {x.code = KeyCode::Char(match c {
                            '0' => ')',
                            '1' => '!',
                            '2' => '@',
                            '3' => '#',
                            '4' => '$',
                            '5' => '%',
                            '6' => '^',
                            '7' => '&',
                            '8' => '*',
                            '9' => '(',
                            '`' => '~',
                            '-' => '_',
                            '=' => '+',
                            ',' => '<',
                            '.' => '>',
                            '/' => '?',
                            '\'' => '"',
                            ';' => ':',
                            '[' => '{',
                            ']' => '}',
                            '\\' => '|',
                            _ => c.to_ascii_uppercase(),
                        });},
                        _ => {return Err(Error::new(ErrorKind::NotFound, "BAD KEY SHIFT"));},
                    };
                    // return Err(Error::new(ErrorKind::Other, format!("{x:?}")));
                }
                if x.modifiers.contains(KeyModifiers::CONTROL) {
                    return Ok(KeyIn(Key::Char(match x.code {
                        KeyCode::Char(c) => match c.to_ascii_lowercase() {
                            '@' => 0u8,
                            'a' => 1,
                            'b' => 2,
                            'c' => 3,
                            'd' => 4,
                            'e' => 5,
                            'f' => 6,
                            'g' => 7,
                            'h' => 8,
                            'i' => 9,
                            'j' => 10,
                            'k' => 11,
                            'l' => 12,
                            'm' => 13,
                            'n' => 14,
                            'o' => 15,
                            'p' => 16,
                            'q' => 17,
                            'r' => 18,
                            's' => 19,
                            't' => 20,
                            'u' => 21,
                            'v' => 22,
                            'w' => 23,
                            'x' => 24,
                            'y' => 25,
                            'z' => 26,
                            '[' => 27,
                            '\\' => 28,
                            ']' => 29,
                            '^' => 30,
                            '_' => 31,
                            _ => {return Err(Error::new(ErrorKind::InvalidData, format!("BAD CTRL CODE {}", c as u8)));},
                        },
                        _ => 255,
                    } as char)));
                }
                Ok(KeyIn(match x.code {
                    KeyCode::Up => Key::ArrowUp,
                    KeyCode::Down => Key::ArrowDown,
                    KeyCode::Left => Key::ArrowLeft,
                    KeyCode::Right => Key::ArrowRight,
                    KeyCode::Backspace => Key::Backspace,
                    KeyCode::Delete => Key::Del,
                    KeyCode::Enter => Key::Enter,
                    KeyCode::Home => Key::Home,
                    KeyCode::End => Key::End,
                    KeyCode::PageUp => Key::PageUp,
                    KeyCode::PageDown => Key::PageDown,
                    KeyCode::Tab => Key::Tab,
                    KeyCode::BackTab => Key::BackTab,
                    KeyCode::Insert => Key::Insert,
                    KeyCode::F(n) => {if n == 1 {return Err(Error::new(ErrorKind::Interrupted, "MANUAL ABORT"))};Key::Unknown},
                    KeyCode::Char(c) => Key::Char(c),
                    KeyCode::Null => Key::Unknown,
                    KeyCode::Esc => Key::Escape,
                    KeyCode::CapsLock => Key::Unknown,
                    KeyCode::ScrollLock => Key::Unknown,
                    KeyCode::NumLock => Key::Unknown,
                    KeyCode::PrintScreen => Key::Unknown,
                    KeyCode::Pause => Key::Unknown,
                    KeyCode::Menu => Key::Unknown,
                    KeyCode::KeypadBegin => Key::Unknown,
                    KeyCode::Media(_) => Key::Unknown,
                    KeyCode::Modifier(m) => match m {
                        ModifierKeyCode::LeftAlt | ModifierKeyCode::RightAlt => Key::Alt,
                        ModifierKeyCode::LeftShift | ModifierKeyCode::RightShift => Key::Shift,
                        ModifierKeyCode::LeftMeta | ModifierKeyCode::RightMeta => Key::Unknown,
                        _ => Key::Unknown,
                    },
                }))
            },
            _ => Ok(Ignore),
        }
        // }() {
        //     Ok(x) => Ok(x),
        //     Err(e) => {let _ = disable_raw_mode();Err(e)},
        // }
        // return self.term.read_key();
    }
    pub fn read_key(&mut self) -> io::Result<Key> {
        loop {
            match self.read_input()? {
                KeyIn(k) => {return Ok(k)},
                _ => {},
            }
        }
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        self.end().unwrap();
        execute!(self.out, PopKeyboardEnhancementFlags).unwrap();
        execute!(self.out, PopKeyboardEnhancementFlags).unwrap();
        execute!(self.out, PopKeyboardEnhancementFlags).unwrap();
    }
}