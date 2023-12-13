use std::time::SystemTime;

pub struct FileMeta {
    pub title: String,
    pub path: String,
    pub histpath: String,
    pub last_modified: SystemTime,
    pub escctrl: bool,
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
// #[derive(Clone, Copy)]
#[allow(non_snake_case)]
pub(crate) mod DArea {
    #![allow(non_upper_case_globals)]
    pub type DArea = u64;
    #[allow(non_camel_case_types)]
    enum Name { // done like this to allow auto updating the flags
        toptext = 0,
        bottext,
        editarea,
        ttall,
        btall,
        eaall,
        btcup,
        btmsg,
        ttsize,
        ttsaved,
        eacul,
        eaculnext,
        eaculnexts,
        eaculprev,
    }
    use Name::*;
    pub const TopText:    DArea = 1<<toptext as u64;
    pub const BotText:    DArea = 1<<bottext as u64;
    pub const EditArea:   DArea = 1<<editarea as u64;
    /// the 'all' flag for top text
    pub const TTAllE:     DArea = 1<<ttall as u64;
    /// top text all
    pub const TTAll:      DArea = TopText | TTSize | TTSaved | TTAllE;
    /// the 'all' flag for bot text
    pub const BTAllE:     DArea = 1<<btall as u64;
    /// bot text all
    pub const BTAll:      DArea = BotText | BTCuP | BTAllE | BTMsg;
    /// the 'all' flag for edit area
    pub const EAAllE:     DArea = 1<<eaall as u64;
    /// edit area all
    pub const EAAll:      DArea = EditArea | EACuL | EACuLNext | EACuLNexts | EACULPrev | EAAllE;
    /// bot text cursor position display
    pub const BTCuP:      DArea = 1<<btcup as u64;
    /// bot text message
    pub const BTMsg:      DArea = 1<<btmsg as u64;
    /// top text size display
    pub const TTSize:     DArea = 1<<ttsize as u64;
    /// top text saved status
    pub const TTSaved:    DArea = 1<<ttsaved as u64;
    /// edit area cur line
    pub const EACuL:      DArea = 1<<eacul as u64;
    /// edit area cur line + next (1) line
    pub const EACuLNext:  DArea = 1<<eaculnext as u64;
    /// edit area cur line + next (all) lines
    pub const EACuLNexts: DArea = 1<<eaculnexts as u64;
    /// edit area cur line + prev (1) line
    pub const EACULPrev:  DArea = 1<<eaculprev as u64;
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
    pub tt_left_len: usize,
    pub bt_left_len: usize,
} impl _Display {pub(crate) fn new()->Self{Self {msg:"THIS IS A BUG, PLEASE CONTACT AUTHOR".to_owned(),lastmod:String::new(),redisplay:0,tt_left_len:0,bt_left_len:0}}}

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