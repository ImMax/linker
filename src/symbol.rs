use std::fmt::{Formatter, Display, Result};

#[derive(Clone)]
pub struct Symbol {
    pub name: String,
    pub value: i64,
    pub seg: usize,
    pub sym_type: String,
    pub module: &'static str,
}

impl Symbol {
    pub fn set_value(&mut self, new_val: i64) {
        self.value = new_val;
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{name:<9} 0x{val:<4X} {seg:<2} {t:<5} {m:<9}",
            name = self.name,
            val  = self.value,
            seg  = self.seg,
            t    = self.sym_type,
            m    = self.module)
    }
}
