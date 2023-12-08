pub(crate) static mut EFLAG: bool = true;
pub(crate) static mut LD_COUNT: u64 = 0;
pub(crate) static TOLERANCE: u64 = 1;
pub(crate) const DEBUGGING: u64 = 
// 7 6 5 4 3 2 1 0
(0b0_0_0_0_0_0_0_0u64 << 56) |
(0b0_0_0_0_0_0_0_0u64 << 48) |
(0b0_0_0_0_0_0_0_0u64 << 40) |
(0b0_0_0_0_0_0_0_0u64 << 32) |
(0b0_0_0_0_0_0_0_0u64 << 24) |
(0b0_0_0_0_0_0_0_0u64 << 16) |
(0b0_0_0_0_0_0_0_1u64 << 8) |
 0b1_0_1_0_0_0_0_0u64;

///
/// registry:
/// - (`0`)  -  Line
/// - (`1`)  -  LineList (Drop)
/// - (`2`)  -  LineList (From)
/// - (`3`)  -  LineIter
/// - (`4`)  -  Controller
/// - (`5`)  -  Controller -- test_readback
/// - (`6`)  -  Controller -- no input loop
/// - (`7`)  -  Controller -- fake save & fake normal exit
/// - (`8`)  -  Controller -- enable content dump
/// - (`9`)  -  Controller -- verbose positioning
/// - (`63`) -  MISC
pub(crate) const fn debugging(x:u64) -> bool {DEBUGGING&(1u64<<x)>0}

/// queries the state of a flag
pub(crate) fn gflag(reg:u64,x:u64) -> bool {
    reg&x>0
}

/// sets a flag
pub(crate) fn sflag(reg:&mut u64,x:u64) -> () {
    *reg|=x;
}

/// clears a flag and returns what it was previously
pub(crate) fn cflag(reg:&mut u64,x:u64) -> bool {
    let b=*reg&x>0;
    *reg&=!x;
    b
}

pub(crate) fn aflag(reg:&mut u64) -> () {
    *reg=!0;
}