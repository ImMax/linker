use std::fmt::{Formatter, Display, Result};

pub const TEXT_SEG: &'static str = ".text";
pub const DATA_SEG: &'static str = ".data";
pub const BSS_SEG: &'static str = ".bss";

#[derive(Clone)]
pub struct Segment {
    pub name: String,
    pub address: i32,
    pub length: i64,
    pub code: String
}

impl Segment {
    pub fn add_len(&mut self, len: i64) {
        self.length += len;
    }
    pub fn combine(segs: Vec<Segment>) -> Segment {
        let name = segs[0].name.clone();
        let address = match segs[0].name.as_ref() {
            TEXT_SEG => 0x1000,
            DATA_SEG => 0x1000 * 4,
            BSS_SEG => 0x1000 * 4 + 16,
            &_ => 0xFFFF,
        };
        let length = segs.iter().fold(0, |mut len, s| {len += s.length; len});
        let code = segs[0].code.clone();
        return Segment {name, address, length, code};
    }
}

impl Display for Segment {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{name:<7} 0x{addr:0>4X}   0x{len:0>4X}   {code:<3}",
            name = self.name,
            addr = self.address,
            len  = self.length,
            code = self.code)
    }
}