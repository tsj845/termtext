//! deals with manipulating lines

use std::alloc::{Layout, alloc, dealloc};
use std::ptr::{read, write, copy_nonoverlapping, copy};
use std::mem;
use crate::*;

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
}

#[derive(Clone)]
pub struct LineList {
    head: u64,
    tail: u64,
    size: u64,
    pub total_size: u64,
    pub iterdeath: bool,
}

pub fn line_to_ptr(l: Line) -> u64 {
    return Box::into_raw(Box::new(l)) as u64;
}

pub struct InternalIterator {ptr: u64, len: u64, i: u64}

impl LineList {
    pub fn new() -> Self {
        Self {head: 0, tail: 0, size: 0, total_size: 0, iterdeath: false}
    }
    pub fn size(&self) -> u64 {self.size}
    pub fn insert(&self, laddr: u64, index: u64) -> () {
        unsafe {
            let saddr: u64 = (self as *const Self) as u64;
            // increment size
            write((saddr + 16) as *mut u64, read((saddr + 16) as *const u64) + 1);
            if index == 0 {
                if self.head != 0 {
                    Line::set_prev_a(self.head, laddr);
                    Line::set_next_a(laddr, self.head);
                } else {
                    // update tail
                    write((saddr + 8) as *mut u64, laddr);
                }
                // update head
                write(saddr as *mut u64, laddr);
                return;
            }
            if index == self.size-1 {
                if self.tail != 0 {
                    Line::set_next_a(self.tail, laddr);
                    Line::set_prev_a(laddr, self.tail);
                } else {
                    // update head
                    write(saddr as *mut u64, laddr);
                }
                // update tail
                write((saddr + 8) as *mut u64, laddr);
                return;
            }
            let mut pline: u64 = self.head;
            if index > (self.size-1)/2 {
                pline = self.tail;
                for _ in 0..(self.size-index-1) {
                    pline = Line::get_prev_a(pline);
                }
            } else {
                for _ in 0..(index-1) {
                    pline = Line::get_next_a(pline);
                }
            }
            let nline: u64 = Line::get_next_a(pline);
            Line::set_next_a(pline, laddr);
            Line::set_next_a(laddr, nline);
            Line::set_prev_a(laddr, pline);
            if nline != 0 {
                Line::set_prev_a(nline, laddr);
            }
        }
    }
    pub fn index(&self, i: u64) -> u64 {
        let size = self.size;
        if i >= size {
            panic!("OUT OF BOUNDS INDEX: ATTEMPTED INDEX OF {i} IN LIST OF SIZE {size}");
        }
        let mut ret: u64 = self.head;
        if i <= (size / 2) {
            for _ in 0..i {
                ret = Line::get_next_a(ret);
            }
        } else {
            ret = self.tail;
            for _ in 0..(size-i) {
                ret = Line::get_prev_a(ret);
            }
        }
        return ret;
        // return Line::from_addr(ret);
    }
}

impl InternalIterator {
    pub(crate) fn new(ptr: u64, len: u64) -> Self {
        Self { ptr, len, i: 0 }
    }
}

impl Iterator for InternalIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.len {
            let v = unsafe{read((self.ptr + self.i) as *const u8)};
            self.i += 1;
            return Some(v);
        }
        return None;
    }
}

pub struct WrappedInternIter {n: InternalIterator}
impl IntoIterator for WrappedInternIter {
    type Item=u8;

    type IntoIter=InternalIterator;

    fn into_iter(self) -> Self::IntoIter {
        self.n
    }
}

// prop getters
impl Line {
    pub fn cap(&self) -> u64 {self.cap}
    pub fn len(&self) -> u64 {self.len}
    pub fn get_next(&self) -> u64 {self.nextln}
    pub fn get_prev(&self) -> u64 {self.prevln}
    pub fn get_addr(&self) -> u64 {(self as *const Line) as u64}
    // gets with Line obj address
    pub fn cap_a(laddr: u64) -> u64 {unsafe {read((laddr + 24) as *const u64)}}
    pub fn len_a(laddr: u64) -> u64 {unsafe {read((laddr + 32) as *const u64)}}
    pub fn get_next_a(laddr: u64) -> u64 {unsafe {read(laddr as *const u64)}}
    pub fn get_prev_a(laddr: u64) -> u64 {unsafe {read((laddr + 8) as *const u64)}}
    pub fn get_linenum_a(laddr: u64) -> u64 {unsafe {read((laddr + 40) as *const u64)}}
    fn get_ptr_a(laddr: u64) -> u64 {unsafe {read((laddr + 16) as *const u64)}}
}

// prop setters
impl Line {
    pub fn set_next_a(laddr: u64, n: u64) {unsafe {write(laddr as *mut u64, n);}}
    pub fn set_prev_a(laddr: u64, n: u64) {unsafe {write((laddr + 8) as *mut u64, n);}}
    fn set_ptr_a(laddr: u64, ptr: u64) {unsafe {write((laddr + 16) as *mut u64, ptr);}}
    fn set_cap_a(laddr: u64, n: u64) {unsafe {write((laddr + 24) as *mut u64, n);}}
    fn set_len_a(laddr: u64, n: u64) {unsafe {write((laddr + 32) as *mut u64, n);}}
    pub fn set_next(&self, n: u64) {Line::set_next_a(self.get_addr(), n)}
    pub fn set_prev(&self, n: u64) {Line::set_prev_a(self.get_addr(), n)}

    pub fn iter_over(laddr: u64) -> WrappedInternIter {
        return WrappedInternIter{n:InternalIterator::new(Line::get_ptr_a(laddr), Line::len_a(laddr))};
    }

    fn resize(laddr: u64, mut n: u64) -> u64 {
        n = match n % 2 == 0 {true=>n,_=>n+1}; // ensure `n` has 2-byte alignment
        let il: u64 = Line::len_a(laddr);
        let ic: u64 = Line::cap_a(laddr);
        if n < il { // ensure data is not lost
            panic!("CANNOT REDUCE SIZE TO BELOW LENGTH");
        }
        if n == ic { // ignore changes where capacity is the same
            return Line::get_ptr_a(laddr);
        }
        let np: *mut u8 = Line::alloc(n, 2);
        let op: *mut u8 = Line::get_ptr_a(laddr) as *mut u8;
        if op as u64 == 0 { // if resizing from null pointer
            Line::set_cap_a(laddr, n); // update capacity
            Line::set_ptr_a(laddr, np as u64); // update ptr property
            return np as u64;
        }
        unsafe {
            copy_nonoverlapping(op, np, il as usize); // move the data
        }
        Line::set_cap_a(laddr, n); // update capacity
        Line::dealloc(op, ic, 2); // deallocate old memory
        Line::set_ptr_a(laddr, np as u64); // update ptr property
        return np as u64;
    }
}

// str manipulation
impl Line {
    /// splits this line in place using the given index, returns the address of a new line containing all characters after the split
    pub fn split_a(laddr: u64, index: u64) -> u64 {
        unsafe {
            let ptr: u64 = Line::get_ptr_a(laddr);
            let len: u64 = Line::len_a(laddr);
            if ptr == 0 || len == 0 {return line_to_ptr(Line::new());}
            // all content should be in the new line
            if index == 0 {
                let naddr: u64 = line_to_ptr(Line::new());
                // copy len, cap, and ptr to the new line
                Line::set_cap_a(naddr, Line::cap_a(laddr));
                Line::set_len_a(naddr, len);
                Line::set_ptr_a(naddr, ptr);
                // zero the len, cap, and ptr of this line
                Line::set_cap_a(laddr, 0);
                Line::set_len_a(laddr, 0);
                Line::set_ptr_a(laddr, 0);
                return naddr;
            }
            // all content should remain in this line
            if index >= len {
                return line_to_ptr(Line::new());
            }
            // if execution reaches this point, then a non-trivial split must be performed
            Line::set_len_a(laddr, len-index);
            let mut s = len - index;
            s = match s % 2 == 0 {true=>s,_=>s+1}; // ensure `n` has 2-byte alignment
            let nptr: u64 = Line::alloc(s, 2) as u64;
            for a in 0..(len-index) {
                write((nptr + a) as *mut u8, read((ptr + a + index) as *const u8));
            }
            return line_to_ptr(Line {nextln:0,prevln:0,ptr:nptr,cap:s,len:len-index});
        }
    }
    /// CHANGES TO THE RETURNED `String` WILL NOT BE RELFLECTED IN THE `Line`
    pub fn to_string_a(laddr: u64) -> String {
        unsafe {
            let ptr: *const u8 = Line::get_ptr_a(laddr) as *const u8; // get the pointer
            let len: usize = Line::len_a(laddr) as usize; // get length
            if len == 0 || (ptr as u64) == 0 {return String::new();} // guard
            String::from_iter(std::slice::from_raw_parts::<u8>(ptr, len).into_iter().map(|n:&u8| (*n) as char))
        }
    }
    pub fn substr_a(laddr: u64, start: u64, end: u64) -> String {
        unsafe {
            let ptr: *const u8 = (Line::get_ptr_a(laddr)+start) as *const u8; // get the pointer
            let len: u64 = Line::len_a(laddr); // get length
            let tlen: usize = (end - start) as usize;
            if tlen > len as usize {panic!("BAD LENGTH");}
            if start >= len || end > len {panic!("OUT OF BOUNDS");}
            if tlen == 0 {return String::new();} // guard
            String::from_iter(std::slice::from_raw_parts::<u8>(ptr, tlen).into_iter().map(|n:&u8| (*n) as char))
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
            let mut ptr: u64 = Line::get_ptr_a(laddr);
            if ptr == 0 {Line::resize(laddr, 1);}
            let ic = Line::cap_a(laddr);
            let il = Line::len_a(laddr);
            if ic == il {
                ptr = Line::resize(laddr, ic * 2);
            }
            write((ptr + il as u64) as *mut u8, c);
            Line::set_len_a(laddr, il+1);
        }
    }
    pub fn pop_char_a(laddr: u64) -> u8 {
        unsafe {
            let ptr = Line::get_ptr_a(laddr); // get pointer
            if ptr == 0 {return 0;} // ensure pointer is not null
            let ic = Line::cap_a(laddr); // capacity and length
            let il = Line::len_a(laddr);
            let c: u8 = read((ptr + il - 1) as *const u8); // read the character
            Line::set_len_a(laddr, il - 1); // update length
            if (il - 1) < (ic / 4) { // if length is now under 1/4 of capacity, resize
                Line::resize(laddr, ic / 2);
            }
            return c;
        }
    }
    pub fn insert_char_a(laddr: u64, idx: u64, c: u8) {
        let il: u64 = Line::len_a(laddr);
        if idx > il {return;}
        let ic: u64 = Line::cap_a(laddr);
        let mut ptr: u64 = Line::get_ptr_a(laddr);
        Line::set_len_a(laddr, il + 1);
        unsafe {
            if ptr == 0 {ptr = Line::resize(laddr, 2);write(ptr as *mut u8, c);return;}
            if  ic == il { // more efficient to resize using code here than to do extra memcopy's
                let np: u64 = Line::alloc(ic * 2, 2) as u64;
                if idx != 0 {copy_nonoverlapping(ptr as *const u8, np as *mut u8, idx as usize);} // don't do zero length copy
                write((np + idx) as *mut u8, c);
                if idx != il {copy_nonoverlapping((ptr + idx) as *const u8, (np + idx + 1) as *mut u8, (il - idx) as usize);} // don't do zero length copy
                Line::dealloc(ptr as *mut u8, ic, 2);
                Line::set_ptr_a(laddr, np);
                Line::set_cap_a(laddr, ic * 2);
            } else {
                copy((ptr + idx) as *const u8, (ptr + idx + 1) as *mut u8, (il - idx) as usize);
                write((ptr + idx) as *mut u8, c);
            }
        }
    }
    pub fn remove_char_a(laddr: u64, idx: u64) -> u8 {
        let il: u64 = Line::len_a(laddr);
        if idx >= il {return 0;}
        if idx == il - 1 {
            return Line::pop_char_a(laddr);
        }
        unsafe {
            Line::set_len_a(laddr, il - 1);
            let ic: u64 = Line::cap_a(laddr);
            let ptr: u64 = Line::get_ptr_a(laddr);
            let c: u8 = read((ptr + idx) as *const u8);
            if il - 1 < ic / 4 {
                let np: u64 = Line::alloc(ic / 2, 2) as u64;
                if idx != 0 {copy_nonoverlapping(ptr as *const u8, np as *mut u8, idx as usize);}
                copy_nonoverlapping((ptr + idx + 1) as *const u8, (np + idx) as *mut u8, (il - idx) as usize);
                Line::dealloc(ptr as *mut u8, ic, 2);
                Line::set_cap_a(laddr, ic / 2 + match (ic / 2) % 2 == 0 {true=>0,_=>1});
                Line::set_ptr_a(laddr, np);
                return c;
            }
            for i in (idx+1)..il {
                write((ptr + i - 1) as *mut u8, read((ptr + i) as *const u8));
            }
            return c;
        }
    }
    // adaptors for Line objects instead of pointer values
    pub fn to_string(self) -> String {Line::to_string_a(self.get_addr())}
    pub fn get_char(&self, idx: u64) -> u8 {Line::get_char_a(self.get_addr(), idx)}
    pub fn set_char(&self, idx: u64, c: u8) {Line::set_char_a(self.get_addr(), idx, c);}
    pub fn push_char(&self, c: u8) {Line::push_char_a(self.get_addr(), c);}
    pub fn pop_char(&self) -> u8 {Line::pop_char_a(self.get_addr())}
    pub fn insert_char(&self, idx: u64, c: u8) {Line::insert_char_a(self.get_addr(), idx, c);}
    pub fn remove_char(&self, idx: u64) -> u8 {Line::remove_char_a(self.get_addr(), idx)}
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
    pub fn new() -> Line {Line {nextln: 0, prevln: 0, ptr: 0, cap: 0, len: 0}}
    pub fn from_addr(addr: u64) -> Line {
        unsafe {
            return read(addr as *const Line);
        }
    }
    pub fn new_with_np(nextln: u64, prevln: u64) -> Line {Line {nextln, prevln, ptr: 0, cap: 0, len: 0}}
    pub fn from_str_with_np(nextln: u64, prevln: u64, s: &str) -> Line {
        if s.len() == 0 {
            return Line::new();
        }
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
        Line {nextln, prevln, ptr: ptr as u64, cap, len}
    }
    pub fn from_str(s: &str) -> Line {Line::from_str_with_np(0, 0, s)}
}

impl Drop for Line {
    fn drop(&mut self) -> () {
        unsafe {
            if debugging(0) && EFLAG {
                let a = (&*self as *const Line) as u64;
                println!("DROPPING: {:x}, {}", a, Line::cap_a(a) + match self.cap % 2 == 0 {true=>0,_=>1});
                if LD_COUNT >= TOLERANCE {
                    panic!("LD_COUNT EXCEDED TOLERANCE");
                } else {
                    LD_COUNT += 1;
                }
            }
        }
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

impl std::fmt::Debug for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Line").field("nextln", &format!("{:#x}",self.nextln)).field("prevln", &format!("{:#x}",self.prevln)).field("ptr", &format!("{:#x}",self.ptr)).field("cap", &self.cap).field("len", &self.len).finish()
    }
}

impl Drop for LineList {
    fn drop(&mut self) {
        if self.iterdeath {
            if debugging(1) {println!("NO DROP, ITER");}
            return;
        }
        let o = unsafe {EFLAG};
        unsafe {
            EFLAG = false;
        }
        if debugging(1) {println!("AFTER WRITE");}
        if self.size != 0 {
            if self.size == 1 {
                mem::drop(Line::from_addr(self.head));
            }
        } else {
            let mut i: u64 = 0;
            loop {
                let a = self.head;
                self.head = Line::get_next_a(a);
                mem::drop(Line::from_addr(a));
                if i >= self.size {break;}
                i += 1;
            }
        }
        unsafe {
            EFLAG = o;
        }
    }
}

impl IntoIterator for LineList {
    type Item = u64;

    type IntoIter = LineIter;

    fn into_iter(mut self) -> Self::IntoIter {
        self.iterdeath = true;
        LineIter::faddr(self.head)
    }
}

impl FromIterator<String> for LineList {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut build: Self = Self {head: 0, tail: 0, size: 0, total_size: 0, iterdeath: false};
        let mut iter = iter.into_iter();
        let mut cur = iter.next();
        if cur.is_some() {
            if debugging(2) {println!("{}", cur.as_ref().unwrap());}
            build.total_size = cur.as_ref().unwrap().len() as u64;
            build.head = Box::into_raw(Box::new(Line::from_str(&cur.unwrap()))) as u64;
            if debugging(2) {println!("POST CONVERT ({:x}): {}", build.head, Line::to_string_a(build.head));}
            build.tail = build.head;
            build.size = 1;
            loop {
                cur = iter.next();
                if cur.is_none() {break;}
                build.size += 1;
                build.total_size += cur.as_ref().unwrap().len() as u64;
                if debugging(2) {println!("{}", cur.as_ref().unwrap());}
                let a: u64 = line_to_ptr(Line::from_str(&cur.unwrap()));
                if debugging(2) {println!("POST CONVERT ({:x}): {}", a, Line::to_string_a(a));}
                Line::set_next_a(build.tail, a);
                Line::set_prev_a(a, build.tail);
                build.tail = a;
            }
        }
        return build;
    }
}

impl FromIterator<std::io::Result<String>> for LineList {
    fn from_iter<T: IntoIterator<Item = std::io::Result<String>>>(iter: T) -> Self {
        let iter = iter.into_iter();
        return Self::from_iter(iter.map(|r:std::io::Result<String>|->String{r.unwrap()}));
    }
}
pub struct LineIter {
    l: u64,
}

impl LineIter {
    pub fn faddr(l: u64) -> Self {
        Self {l}
    }
}

impl Iterator for LineIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.l == 0 {
            if debugging(3) {println!("ITER ENDED");}
            return None;
        }
        if debugging(3) {println!("ITER CUR ADDR: {:x}", self.l);}
        let x: u64 = self.l;
        self.l = Line::get_next_a(self.l);
        if debugging(3) {println!("ITER NEXT ADDR: {:x}", self.l);}
        return Some(x);
    }
}