use std::time::SystemTime;

pub struct FileMeta {
    pub title: String,
    pub path: String,
    pub histpath: String,
    pub last_modified: SystemTime,
} impl FileMeta {
    pub(crate) fn fmt_last_modified(&self) -> String {
        let last_modified: u64 = SystemTime::now().duration_since(self.last_modified).unwrap().as_secs();
        let years: u64 = last_modified / 31536000;
        let rweeks = last_modified % 31536000;
        let weeks: u64 = rweeks / 640800;
        let rdays = rweeks % 640800;
        let days: u64 = rdays / 86400;
        let rhours = rdays % 86400;
        let hours: u64 = rhours / 3600;
        let rminutes = rhours % 3600;
        let minutes: u64 = rminutes / 60;
        let seconds: u64 = rminutes % 60;
        // if debugging(63) {
        //     return format!("({}:{})y\n({}:{})w\n({}:{})d\n({}:{})h\n({}:{})m\n({})s", last_modified, years, rweeks, weeks, rdays, days, rhours, hours, rminutes, minutes, seconds);
        // }
        return format!("{}y-{}w-{}d-{}h-{}m-{}s", years, weeks, days, hours, minutes, seconds);
    }
}

pub(crate) struct _MoveRestrict {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub up_max: u64,
    pub down_max: u64,
    pub left_max: u64,
    pub right_max: u64,
}impl _MoveRestrict {pub(crate) fn new()->Self{Self { up: true, down: true, left: true, right: true, up_max: 0, down_max: 0, left_max: 0, right_max: 0 }}}

/// display areas
#[derive(Clone, Copy)]
pub(crate) enum DArea {
    TopText = 0,
    BotText = 1,
    EditArea = 2,
    TTAll = 3,
    BTAll = 4,
    EAAll = 5,
    BTCuP = 6,
    TTSaved,
    EACuL,
    EACuLNext,
    EACuLNexts,
    EACULPrev,
}
impl DArea {
    pub(crate) fn allflags(a: Self) -> u64 {
        match a {
            Self::TTAll => 0b10000000u64,
            Self::BTAll => 0b100000u64,
            Self::EAAll => 0b11110000000u64,
            _ => 0u64,
        }
    }
}

pub(crate) struct _Display {
    pub msg: String,
    pub lastmod: String,
    /// registry:
    /// #### main sections
    /// - (`0`) - top text
    /// - (`1`) - bottom text
    /// - (`2`) - editable area
    /// #### subsections
    /// - (`3`)  - top text - blanket
    /// - (`4`)  - bottom text - blanket
    /// - (`5`)  - editable area - blanket
    /// - (`6`)  - bottom text - cur pos
    /// - (`7`)  - top text - saved
    /// - (`8`)  - EA - cur line only
    /// - (`9`)  - EA - cur + next line only
    /// - (`10`) - EA - cur + next lines
    /// - (`11`) - EA - cur + prev line only
    pub redisplay: u64,
    pub top_text_left_length: usize,
    pub bot_text_left_length: usize,
} impl _Display {pub(crate) fn new()->Self{Self {msg:String::new(),lastmod:String::new(),redisplay:0,top_text_left_length:0,bot_text_left_length:0}}}

pub(crate) struct Attrs {
    /// rows cols
    pub size: (u64, u64),
    /// rows cols
    pub frame_start: (u64, u64),
    /// row, col
    pub pos: (u64, u64),
    pub pref_x: u64,
    pub mov_restrict: _MoveRestrict,
    pub suppress_move_errs: bool,
    pub display: _Display,
}