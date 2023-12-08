use std::io::{Error, ErrorKind, Write};
use std::io::{Stdout, stdout};

use crossterm::cursor::*;
use crossterm::event::*;
use crossterm::terminal::*;
use crossterm::{queue, execute};
use std::io;

pub use crossterm::cursor::position;

use console::Term as Terminal;

pub fn unfckterminal() -> () {
    let _ = enable_raw_mode();
    {
        let mut out = stdout();
        let _ = execute!(out, DisableMouseCapture);
        let _ = execute!(out, PopKeyboardEnhancementFlags);
        let _ = execute!(out, PopKeyboardEnhancementFlags);
        let _ = execute!(out, PopKeyboardEnhancementFlags);
    }
    let _ = disable_raw_mode();
    let _ = Terminal::stdout().show_cursor();
}

pub fn apply_key_ctrl(k: KeyCode) -> io::Result<KeyCode> {
    return Ok(match k {
        KeyCode::Char(c) => KeyCode::Char(match c.to_ascii_lowercase() {
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
            _ => c as u8,
            // _ => {return Err(Error::new(ErrorKind::InvalidData, format!("BAD CTRL CODE {}", c as u8)));},
        } as char),
        _ => k,
        // _ => 255,
    });
}

pub fn apply_key_shift(k: KeyCode) -> io::Result<KeyCode> {
    match k {
        KeyCode::Char(c) => {return Ok(KeyCode::Char(match c {
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
        }));},
        _ => {return Ok(k);}
        // _ => {return Err(Error::new(ErrorKind::NotFound, "BAD KEY SHIFT"));},
    };
}

/// row, column
pub type Position = (u64, u64);

macro_rules! command {
    ($self:ident, $($com:expr),+) => {
        match $self._queue {true=>queue!($($com),+), false=>execute!($($com),+)}
    };
}

pub struct Term {
    pub out: Stdout,
    pub term: Terminal,
    _queue: bool,
    _raw: bool,
    _oraw: bool,
}

impl Term {
    pub fn new() -> Self {
        let mut x = Self {out:stdout(),term:Terminal::stdout(),_queue:false,_raw:false,_oraw:false};
        execute!(x.out, PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES)).unwrap();
        execute!(x.out, PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES)).unwrap();
        execute!(x.out, PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_ALTERNATE_KEYS)).unwrap();
        return x;
    }
    pub fn queue(&mut self) -> () {self._queue=true;}
    pub fn flush(&mut self) -> () {self._queue=false;self.out.flush().unwrap();}
    pub fn save_raw(&mut self) -> () {self._oraw=self._raw;self.end().unwrap();}
    pub fn restore_raw(&mut self) -> () {if self._oraw {self.begin().unwrap();}}
    pub fn begin(&mut self) -> io::Result<()> {
        if self._raw {
            return Ok(());
        }
        self._raw = true;
        execute!(self.out, EnableMouseCapture)?;
        enable_raw_mode()
    }
    pub fn end(&mut self) -> io::Result<()> {
        if !self._raw {
            return Ok(());
        }
        self._raw = false;
        execute!(self.out, DisableMouseCapture)?;
        disable_raw_mode()
    }
    pub fn cleanup(&mut self) -> io::Result<()> {
        self.end()?;
        execute!(self.out, PopKeyboardEnhancementFlags).unwrap();
        execute!(self.out, PopKeyboardEnhancementFlags).unwrap();
        execute!(self.out, PopKeyboardEnhancementFlags).unwrap();
        Ok(())
    }
    #[cfg_attr(do_inline, inline(always))]
    pub fn top_left(&mut self) -> () {let _ = command!(self, self.out, MoveTo(0, 0));}
    #[cfg_attr(do_inline, inline(always))]
    pub fn scroll_up(&mut self) -> () {let _ = command!(self, self.out, ScrollUp(1));}
    #[cfg_attr(do_inline, inline(always))]
    pub fn scroll_down(&mut self) -> () {let _ = command!(self, self.out, ScrollDown(1));}
    #[cfg_attr(do_inline, inline(always))]
    pub fn set_cur_pos(&mut self, y: u64, x: u64) -> () {let _ = execute!(self.out, MoveTo(x as u16, y as u16));}
    // pub fn set_cur_pos(&mut self, x: u64, y: u64) -> () {let _ = self.term.move_cursor_to(x, y);}
    #[cfg_attr(do_inline, inline(always))]
    pub fn up(&mut self) -> () {let _ = command!(self, self.out, MoveUp(1));}
    #[cfg_attr(do_inline, inline(always))]
    pub fn up_n(&mut self, n: u64) -> () {let _ = command!(self, self.out, MoveUp(n as u16));}
    #[cfg_attr(do_inline, inline(always))]
    pub fn down(&mut self) -> () {let _ = command!(self, self.out, MoveDown(1));}
    #[cfg_attr(do_inline, inline(always))]
    pub fn down_n(&mut self, n: u64) -> () {let _ = command!(self, self.out, MoveDown(n as u16));}
    #[cfg_attr(do_inline, inline(always))]
    pub fn right(&mut self) -> () {let _ = command!(self, self.out, MoveRight(1));}
    #[cfg_attr(do_inline, inline(always))]
    pub fn right_n(&mut self, n: u64) -> () {let _ = command!(self, self.out, MoveRight(n as u16));}
    #[cfg_attr(do_inline, inline(always))]
    pub fn left(&mut self) -> () {let _ = command!(self, self.out, MoveLeft(1));}
    #[cfg_attr(do_inline, inline(always))]
    pub fn left_n(&mut self, n: u64) -> () {let _ = command!(self, self.out, MoveLeft(n as u16));}

    pub fn hide_cursor(&mut self) -> () {self.term.hide_cursor().unwrap();}
    pub fn show_cursor(&mut self) -> () {self.term.show_cursor().unwrap();}
    pub fn size(&mut self) -> Position {
        let s = self.term.size_checked().unwrap();
        (s.0 as u64, s.1 as u64)
    }
    #[cfg_attr(do_inline, inline(always))]
    pub fn reset_row(&mut self) -> () {let _ = command!(self, self.out, MoveToRow(0));}
    #[cfg_attr(do_inline, inline(always))]
    pub fn set_cur_row(&mut self, n: u64) -> () {let _ = command!(self, self.out, MoveToRow(n as u16));}
    #[cfg_attr(do_inline, inline(always))]
    pub fn reset_col(&mut self) -> () {let _ = command!(self, self.out, MoveToColumn(0));}
    #[cfg_attr(do_inline, inline(always))]
    pub fn set_cur_col(&mut self, n: u64) -> () {let _ = command!(self, self.out, MoveToColumn(n as u16));}
    #[cfg_attr(do_inline, inline(always))]
    pub fn clear_screen(&mut self) -> () {let _ = command!(self, self.out, Clear(ClearType::All));self.top_left();}
    #[cfg_attr(do_inline, inline(always))]
    pub fn clear_to_end(&mut self) -> () {let _ = command!(self, self.out, Clear(ClearType::FromCursorDown));}
    // pub fn clear_to_end(&mut self) -> () {let _ = self.out.flush();let _ = self.term.clear_to_end_of_screen();}
    #[cfg_attr(do_inline, inline(always))]
    pub fn clear_line(&mut self) -> () {let _ = execute!(self.out, Clear(ClearType::CurrentLine));print!("\r");}
    #[cfg_attr(do_inline, inline(always))]
    pub fn clear_to_newline(&mut self) -> () {let _ = command!(self, self.out, Clear(ClearType::UntilNewLine));}
    // pub unsafe fn read_input_unsf(&mut self) -> io::Result<Input> {
    //     if !self._raw {
    //         return Err(Error::new(ErrorKind::NotConnected, "RAW MODE NOT ENABLED"));
    //     }
    //     // match || -> io::Result<Input> {
    //     match read()? {
    //         Event::Key(mut x) => {
    //             if x.kind == KeyEventKind::Release {
    //                 return Ok(Ignore);
    //             }
    //             if x.modifiers.contains(KeyModifiers::SHIFT) {
    //                 match x.code {
    //                     KeyCode::Char(c) => {x.code = KeyCode::Char(match c {
    //                         '0' => ')',
    //                         '1' => '!',
    //                         '2' => '@',
    //                         '3' => '#',
    //                         '4' => '$',
    //                         '5' => '%',
    //                         '6' => '^',
    //                         '7' => '&',
    //                         '8' => '*',
    //                         '9' => '(',
    //                         '`' => '~',
    //                         '-' => '_',
    //                         '=' => '+',
    //                         ',' => '<',
    //                         '.' => '>',
    //                         '/' => '?',
    //                         '\'' => '"',
    //                         ';' => ':',
    //                         '[' => '{',
    //                         ']' => '}',
    //                         '\\' => '|',
    //                         _ => c.to_ascii_uppercase(),
    //                     });},
    //                     _ => {return Err(Error::new(ErrorKind::NotFound, "BAD KEY SHIFT"));},
    //                 };
    //             }
    //             if x.modifiers.contains(KeyModifiers::CONTROL) {
    //                 return Ok(KeyIn(Key::Char(match x.code {
    //                     KeyCode::Char(c) => match c.to_ascii_lowercase() {
    //                         '@' => 0u8,
    //                         'a' => 1,
    //                         'b' => 2,
    //                         'c' => 3,
    //                         'd' => 4,
    //                         'e' => 5,
    //                         'f' => 6,
    //                         'g' => 7,
    //                         'h' => 8,
    //                         'i' => 9,
    //                         'j' => 10,
    //                         'k' => 11,
    //                         'l' => 12,
    //                         'm' => 13,
    //                         'n' => 14,
    //                         'o' => 15,
    //                         'p' => 16,
    //                         'q' => 17,
    //                         'r' => 18,
    //                         's' => 19,
    //                         't' => 20,
    //                         'u' => 21,
    //                         'v' => 22,
    //                         'w' => 23,
    //                         'x' => 24,
    //                         'y' => 25,
    //                         'z' => 26,
    //                         '[' => 27,
    //                         '\\' => 28,
    //                         ']' => 29,
    //                         '^' => 30,
    //                         '_' => 31,
    //                         _ => {return Err(Error::new(ErrorKind::InvalidData, format!("BAD CTRL CODE {}", c as u8)));},
    //                     },
    //                     _ => 255,
    //                 } as char)));
    //             }
    //             Ok(KeyIn(match x.code {
    //                 KeyCode::Down => Key::ArrowUp,
    //                 KeyCode::Up => Key::ArrowDown,
    //                 KeyCode::Left => Key::ArrowLeft,
    //                 KeyCode::Right => Key::ArrowRight,
    //                 KeyCode::Backspace => Key::Backspace,
    //                 KeyCode::Delete => Key::Del,
    //                 KeyCode::Enter => Key::Enter,
    //                 KeyCode::Home => Key::Home,
    //                 KeyCode::End => Key::End,
    //                 KeyCode::PageUp => Key::PageUp,
    //                 KeyCode::PageDown => Key::PageDown,
    //                 KeyCode::Tab => Key::Tab,
    //                 KeyCode::BackTab => Key::BackTab,
    //                 KeyCode::Insert => Key::Insert,
    //                 KeyCode::F(n) => {if n == 1 {return Err(Error::new(ErrorKind::Interrupted, "MANUAL ABORT"))};Key::Unknown},
    //                 KeyCode::Char(c) => Key::Char(c),
    //                 KeyCode::Null => Key::Unknown,
    //                 KeyCode::Esc => Key::Escape,
    //                 KeyCode::CapsLock => Key::Unknown,
    //                 KeyCode::ScrollLock => Key::Unknown,
    //                 KeyCode::NumLock => Key::Unknown,
    //                 KeyCode::PrintScreen => Key::Unknown,
    //                 KeyCode::Pause => Key::Unknown,
    //                 KeyCode::Menu => Key::Unknown,
    //                 KeyCode::KeypadBegin => Key::Unknown,
    //                 KeyCode::Media(_) => Key::Unknown,
    //                 KeyCode::Modifier(m) => match m {
    //                     ModifierKeyCode::LeftAlt | ModifierKeyCode::RightAlt => Key::Alt,
    //                     ModifierKeyCode::LeftShift | ModifierKeyCode::RightShift => Key::Shift,
    //                     ModifierKeyCode::LeftMeta | ModifierKeyCode::RightMeta => Key::Unknown,
    //                     _ => Key::Unknown,
    //                 },
    //             }))
    //         },
    //         Event::Mouse(me) => {
    //             match me.kind {
    //                 MouseEventKind::ScrollUp => {print!("\x1b[T");},
    //                 MouseEventKind::ScrollDown => {print!("\x1b[S");},
    //                 _ => {},
    //             };
    //             Ok(MouseIn)
    //         },
    //         _ => Ok(Ignore),
    //     }
    //     // }() {
    //     //     Ok(x) => Ok(x),
    //     //     Err(e) => {let _ = disable_raw_mode();Err(e)},
    //     // }
    //     // return self.term.read_key();
    // }
    // pub fn read_key(&mut self) -> io::Result<KeyEvent> {
    //     loop {
    //         if self._iqueue.read().unwrap().has().0 {
    //             let mut x = self._iqueue.write().unwrap().read_key();
    //             if x.kind == KeyEventKind::Release {
    //                 continue;
    //             }
    //             if x.modifiers.contains(KeyModifiers::SHIFT) {
    //                 match x.code {
    //                     KeyCode::Char(c) => {x.code = KeyCode::Char(match c {
    //                         '0' => ')',
    //                         '1' => '!',
    //                         '2' => '@',
    //                         '3' => '#',
    //                         '4' => '$',
    //                         '5' => '%',
    //                         '6' => '^',
    //                         '7' => '&',
    //                         '8' => '*',
    //                         '9' => '(',
    //                         '`' => '~',
    //                         '-' => '_',
    //                         '=' => '+',
    //                         ',' => '<',
    //                         '.' => '>',
    //                         '/' => '?',
    //                         '\'' => '"',
    //                         ';' => ':',
    //                         '[' => '{',
    //                         ']' => '}',
    //                         '\\' => '|',
    //                         _ => c.to_ascii_uppercase(),
    //                     });},
    //                     _ => {return Err(Error::new(ErrorKind::NotFound, "BAD KEY SHIFT"));},
    //                 };
    //                 // return Err(Error::new(ErrorKind::Other, format!("{x:?}")));
    //             }
    //             if x.modifiers.contains(KeyModifiers::CONTROL) {
    //                 x.code = KeyCode::Char(match x.code {
    //                     KeyCode::Char(c) => match c.to_ascii_lowercase() {
    //                         '@' => 0u8,
    //                         'a' => 1,
    //                         'b' => 2,
    //                         'c' => 3,
    //                         'd' => 4,
    //                         'e' => 5,
    //                         'f' => 6,
    //                         'g' => 7,
    //                         'h' => 8,
    //                         'i' => 9,
    //                         'j' => 10,
    //                         'k' => 11,
    //                         'l' => 12,
    //                         'm' => 13,
    //                         'n' => 14,
    //                         'o' => 15,
    //                         'p' => 16,
    //                         'q' => 17,
    //                         'r' => 18,
    //                         's' => 19,
    //                         't' => 20,
    //                         'u' => 21,
    //                         'v' => 22,
    //                         'w' => 23,
    //                         'x' => 24,
    //                         'y' => 25,
    //                         'z' => 26,
    //                         '[' => 27,
    //                         '\\' => 28,
    //                         ']' => 29,
    //                         '^' => 30,
    //                         '_' => 31,
    //                         _ => {return Err(Error::new(ErrorKind::InvalidData, format!("BAD CTRL CODE {}", c as u8)));},
    //                     },
    //                     _ => 255,
    //                 } as char);
    //             }
    //         }
    //     }
    //     // loop {
    //     //     match self.read_input()? {
    //     //         KeyIn(k) => {return Ok(k)},
    //     //         _ => {},
    //     //     }
    //     // }
    // }
}

impl Drop for Term {
    fn drop(&mut self) {
        self.end().unwrap();
        self.cleanup().unwrap();
    }
}