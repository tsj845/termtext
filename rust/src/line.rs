//! deals with manipulating lines

use std::alloc::{Layout, alloc, dealloc};
use std::ptr::{read, write, copy_nonoverlapping};

/// represents a line of text with built in linked list functionality
pub struct Line {
    /// pointer to next line
    nextln: u64,
    /// pointer to previous line
    prevln: u64,
    /// the pointer to this line's allocated memory, the size of this memory can be found in [cap](Line::cap)
    ptr: u64,
    /// this line's capacity
    cap: u64,
    /// this line's length
    len: u64,
    /// the line number
    line_num: u64
}

impl std::fmt::Debug for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Line").field("nextln", &format!("{:#x}",self.nextln)).field("prevln", &format!("{:#x}",self.prevln)).field("ptr", &format!("{:#x}",self.ptr)).field("cap", &self.cap).field("len", &self.len).field("line_num", &self.line_num).finish()
    }
}

// prop getters
impl Line {
    /// property accessor for `Line::cap`
    pub fn cap(&self) -> u64 {self.cap}
    /// property accessor for `Line::len`
    pub fn len(&self) -> u64 {self.len}
    /// property accessor for `Line::nextln`
    pub fn get_next(&self) -> u64 {self.nextln}
    /// property accessor for `Line::prevln`
    pub fn get_prev(&self) -> u64 {self.prevln}
    /// property accessor for `Line::len`
    pub fn get_lnum(&self) -> u64 {self.line_num}
    /// returns the memory address that this `Line` object is stored at
    pub fn get_addr(&self) -> u64 {(self as *const Line) as u64}
    // get with Line obj address
    /// version of `Line::cap` that takes a pointer
    pub fn cap_a(laddr: u64) -> u64 {unsafe {read((laddr + 24) as *const u64)}}
    /// version of `Line::len` that takes a pointer
    pub fn len_a(laddr: u64) -> u64 {unsafe {read((laddr + 32) as *const u64)}}
    /// version of `Line::get_next` that takes a pointer
    pub fn get_next_a(laddr: u64) -> u64 {unsafe {read(laddr as *const u64)}}
    /// version of `Line::get_prev` that takes a pointer
    pub fn get_prev_a(laddr: u64) -> u64 {unsafe {read((laddr + 8) as *const u64)}}
    /// version of `Line::get_lnum` that takes a pointer
    pub fn get_lnum_a(laddr: u64) -> u64 {unsafe {read((laddr + 40) as *const u64)}}
    /// gets the `Line::ptr` property
    /// 
    /// FOR INTERNAL USE ONLY
    fn get_ptr_a(laddr: u64) -> u64 {unsafe {read((laddr + 16) as *const u64)}}
}

// prop setters
impl Line {
    /// set the nextln property using a pointer
    pub fn set_next_a(laddr: u64, n: u64) {unsafe {write(laddr as *mut u64, n);}}
    /// set the prevln property using a pointer
    pub fn set_prev_a(laddr: u64, n: u64) {unsafe {write((laddr + 8) as *mut u64, n);}}
    /// sets the `Line::ptr` property
    /// 
    /// FOR INTERNAL USE ONLY
    fn set_ptr_a(laddr: u64, ptr: u64) {unsafe {write((laddr + 16) as *mut u64, ptr);}}
    fn set_cap_a(laddr: u64, n: u64) {unsafe {write((laddr + 24) as *mut u64, n);}}
    fn set_len_a(laddr: u64, n: u64) {unsafe {write((laddr + 32) as *mut u64, n);}}
    // adaptors
    /// set the nextln property
    pub fn set_next(&self, n: u64) {Line::set_next_a(self.get_addr(), n)}
    /// set the prevln property
    pub fn set_prev(&self, n: u64) {Line::set_prev_a(self.get_addr(), n)}

    fn resize(laddr: u64, n: u64) {
        let il: u64 = Line::len_a(laddr);
        let ic: u64 = Line::cap_a(laddr);
        if n < il {
            panic!("CANNOT REDUCE SIZE TO BELOW LENGTH");
        }
        let np: *mut u8 = Line::alloc(n, 2);
        let op: *mut u8 = Line::get_ptr_a(laddr) as *mut u8;
        unsafe {
            copy_nonoverlapping(op, np, il as usize);
        }
        Line::set_cap_a(laddr, n);
        Line::dealloc(op, ic, 2);
        Line::set_ptr_a(laddr, np as u64);
    }
}

// str manipulation
impl Line {
    pub fn to_string_a(laddr: u64) -> String {
        unsafe {
            let ptr: *mut u8 = read((laddr + 16) as *const *mut u8) as *mut u8;
            let cap: usize = read((laddr + 24) as *const usize);
            let len: usize = read((laddr + 32) as *const usize);
            if len == 0 {return String::new();} // guard
            String::from_iter(std::slice::from_raw_parts::<u8>(ptr as *const u8, cap).into_iter().map(|n:&u8| (*n) as char))
        }
    }
    pub fn get_char_a(laddr: u64, idx: u64) -> u8 {
        unsafe {
            if Line::len_a(laddr) <= idx {return 0;}
            return read((Line::get_ptr_a(laddr) + idx) as *const u8);
        }
    }
    pub fn set_char_a(laddr: u64, idx: u64, c: u8) {
        unsafe {
            if Line::len_a(laddr) <= idx {return;}
            write((Line::get_ptr_a(laddr) + idx) as *mut u8, c);
        }
    }
    pub fn push_char_a(laddr: u64, c: u8) {
        unsafe {
            if Line::get_ptr_a(laddr) == 0 {Line::set_ptr_a(laddr, Line::alloc(2, 2) as u64);Line::set_cap_a(laddr, 2);}
            let ic = Line::cap_a(laddr);
            let il = Line::len_a(laddr);
            if ic == il {
                Line::resize(laddr, ic * 2);
            }
            let ptr: u64 = Line::get_ptr_a(laddr);
            write((ptr + il as u64) as *mut u8, c);
            Line::set_len_a(laddr, il+1);
        }
    }
    // adaptors for Line objects instead of pointer values
    pub fn to_string(&self) -> String {Line::to_string_a(self.get_addr())}
    pub fn get_char(&self, idx: u64) -> u8 {Line::get_char_a(self.get_addr(), idx)}
    pub fn set_char(&self, idx: u64, c: u8) {Line::set_char_a(self.get_addr(), idx, c)}
}

// basic things
impl Line {
    fn alloc(size: u64, align: u64) -> *mut u8 {
        unsafe {
            let p = alloc(Layout::from_size_align_unchecked(size as usize, align as usize));
            if p as usize == 0 {
                panic!("FAILURE");
            }
            p
        }
    }
    fn dealloc(ptr: *mut u8, size: u64, align: u64) {
        unsafe {
            dealloc(ptr, Layout::from_size_align_unchecked(size as usize, align as usize))
        }
    }
    // various constructors
    pub fn new() -> Line {Line {nextln: 0, prevln: 0, ptr: 0, cap: 0, len: 0, line_num: 0}}
    pub fn new_with_np(nextln: u64, prevln: u64) -> Line {Line {nextln, prevln, ptr: 0, cap: 0, len: 0, line_num: 0}}
    pub fn from_str_with_np(nextln: u64, prevln: u64, s: &str) -> Line {
        let n: String = s.to_owned();
        let len: u64 = n.len() as u64;
        let ptr: *mut u8 = Line::alloc(len, 2);
        let cap = match len % 2 == 0 {true=>len,_=>len+1};
        unsafe {
            let mut i = 0usize;
            for b in n.as_bytes() {
                *((ptr as usize + i) as *mut u8) = *b;
                i += 1;
            }
        }
        Line {nextln, prevln, ptr: ptr as u64, cap, len, line_num: 0}
    }
    pub fn from_str(s: &str) -> Line {Line::from_str_with_np(0, 0, s)}
}

impl Drop for Line {
    fn drop(&mut self) -> () {
        // deallocate text
        if self.ptr as usize != 0 {
            Line::dealloc(self.ptr as *mut u8, self.cap, 2);
        }
        // make dangling next pointer be null instead
        if self.prevln != 0 {
            Line::set_next_a(self.get_prev(), 0);
        }
        // make dangling prev pointer be null instead
        if self.nextln != 0 {
            Line::set_prev_a(self.get_next(), 0);
        }
    }
}