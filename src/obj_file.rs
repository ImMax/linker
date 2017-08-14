use std::fmt::{Formatter, Display, Result};
use segment::Segment;
use segment::TEXT_SEG;
use segment::DATA_SEG;
use segment::BSS_SEG;
use symbol::Symbol;
use parser;
use parser::SEGMENTS;
use parser::SYMBOLS;
use std::collections::HashMap;

pub struct ObjectFile {
    pub segments: HashMap<String, Segment>,
    pub symbols: HashMap<String, Symbol>,
}

impl ObjectFile {
    pub fn from_file(p : &'static str) -> ObjectFile {
        let file_map = parser::parse_obj_file(p);
        let mut segments = parser::lines_to_segments(file_map.get(SEGMENTS).unwrap().to_vec());
        let sym_lines = file_map.get(SYMBOLS).unwrap().to_vec();
        let mut symbols = parser::lines_to_symbols(&sym_lines, p);
        parser::do_common_blocks(&mut symbols, &mut segments);
        return ObjectFile {segments, symbols};
    }

    pub fn combine(obj_files: Vec<ObjectFile>) -> ObjectFile {
        let mut text_segs: Vec<Segment> = Vec::new();
        let mut data_segs: Vec<Segment> = Vec::new();
        let mut bss_segs: Vec<Segment> = Vec::new();

        let mut comb_symbs: HashMap<String, Symbol> = HashMap::new();
        for obj in &obj_files {
            text_segs.push(obj.segments[TEXT_SEG].clone());
            data_segs.push(obj.segments[DATA_SEG].clone());
            bss_segs.push(obj.segments[BSS_SEG].clone());

            for (k, v) in obj.symbols.iter() {
                comb_symbs.insert(k.clone(), v.clone());
            }

        }

        let mut comb_segs: HashMap<String, Segment> = HashMap::new();
        comb_segs.insert(TEXT_SEG.to_string(), Segment::combine(text_segs));
        comb_segs.insert(DATA_SEG.to_string(), Segment::combine(data_segs));
        comb_segs.insert(BSS_SEG.to_string(), Segment::combine(bss_segs));

        return ObjectFile { segments: comb_segs, symbols: comb_symbs };
    }
}

impl Display for ObjectFile {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut segs = String::new();
        let mut symbs = String::new();
        let mut mod_name = String::new();

        segs.push_str(&format!("  {}\n", self.segments[TEXT_SEG]));
        segs.push_str(&format!("  {}\n", self.segments[DATA_SEG]));
        segs.push_str(&format!("  {}\n", self.segments[BSS_SEG]));

        for (_, sym) in &self.symbols {
            symbs.push_str(&format!("  {}\n", sym));
            if (sym.sym_type == "*ABS*") {
                mod_name = sym.name.clone();
            }
        }

        write!(f, "       {}\nsegments:\n{}\nsymbols:\n{}", mod_name, segs, symbs)
    }
}
