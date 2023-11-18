pub(crate) static mut EFLAG: bool = true;
pub(crate) static mut LD_COUNT: u64 = 0;
pub(crate) static TOLERANCE: u64 = 1;
///
/// registry:
/// - (`0`) - Line
/// - (`1`) - LineList (Drop)
/// - (`2`) - LineList (From)
/// - (`3`) - LineIter
/// - (`4`) - Controller
/// - (`5`) - Controller -- test_readback
pub(crate) const DEBUGGING: u64 = 
// 7 6 5 4 3 2 1 0
(0b0_0_0_0_0_0_0_0u64 << 56) |
(0b0_0_0_0_0_0_0_0u64 << 48) |
(0b0_0_0_0_0_0_0_0u64 << 40) |
(0b0_0_0_0_0_0_0_0u64 << 32) |
(0b0_0_0_0_0_0_0_0u64 << 24) |
(0b0_0_0_0_0_0_0_0u64 << 16) |
(0b0_0_0_0_0_0_0_0u64 << 8) |
 0b0_0_0_0_0_0_1_0u64;

pub(crate) const fn debugging(x:u64) -> bool {DEBUGGING&(1u64<<x)>0}