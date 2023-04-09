use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;
use super::segment::*;
use super::symbol::*;

pub const SEGMENTS: &'static str = "- segments -";
pub const SYMBOLS: &'static str = "- symbols -";
pub const RELS: &'static str = "- rels -";
pub const DATA: &'static str = "- data -";

// const MAGIC_LINE: usize = 0;
// const ENTRIES_LINE: usize = 1;
//
// const NSEGS_ENTRY: usize = 0;
// const NSYMS_ENTRY: usize = 1;
// const NRELS_ENTRY: usize = 2;

pub fn parse_obj_file(p : &str) -> HashMap<&str, Vec<String>> {
    let path = Path::new(p);
    let mut file = match File::open(&path) {
        Err(why) => panic!("can't open file {}", why.to_string()),
        Ok(file) => file
    };
    let mut content = String::new();
    let file_string = match file.read_to_string(&mut content) {
        Err(why) => panic!("couldn't read {}", why.to_string()),
        Ok(_) => content,
    };
    let lines = file_string.lines().collect::<Vec<&str>>().clone();
    return to_file_map(&lines).clone();
}

fn to_file_map<'a>(lines: & [&str]) -> HashMap<&'a str, Vec<String>> {
    let mut cur = SEGMENTS;
    let mut file_map: HashMap<&'a str, Vec<String>> = HashMap::new();
    file_map.insert(SEGMENTS, Vec::new());
    file_map.insert(SYMBOLS, Vec::new());
    file_map.insert(RELS, Vec::new());
    file_map.insert(DATA, Vec::new());
    for &l in &lines[2..] {
        match l {
            SEGMENTS => cur = SEGMENTS,
            SYMBOLS  => cur = SYMBOLS,
            RELS     => cur = RELS,
            DATA     => cur = DATA,
            _ => file_map.get_mut(cur).unwrap().push(l.to_string()),
        }
    }
    return file_map;
}

// fn get_nsegs(lines: &Vec<&str>) -> usize {
//     let entries_nums = lines[ENTRIES_LINE].split_whitespace().collect::<Vec<&str>>();
//     return entries_nums[NSEGS_ENTRY].parse::<usize>().unwrap();
// }

// fn get_nsyms(lines: &Vec<&str>) -> usize {
//     let entries_nums = lines[ENTRIES_LINE].split_whitespace().collect::<Vec<&str>>();
//     return entries_nums[NSYMS_ENTRY].parse::<usize>().unwrap();
// }

fn str_to_segment(s: &str) -> Segment {
    let mut x = s.split_whitespace();
    return Segment {
        name: x.next().unwrap().to_string(),
        address: x.next().unwrap().parse::<i32>().unwrap(),
        length: i64::from_str_radix(x.next().unwrap(), 16).unwrap(),
        code: x.next().unwrap().to_string()
    };
}

fn str_to_symbol(s: &str, m: &'static str) -> Symbol {
    let mut x = s.split_whitespace();
    return Symbol {
        name: x.next().unwrap().to_string(),
        value: x.next().unwrap().parse::<i64>().unwrap(),
        seg: x.next().unwrap().parse::<usize>().unwrap(),
        sym_type: x.next().unwrap().to_string(),
        module: m
    };
}

pub fn lines_to_segments(lines: Vec<String>) -> HashMap<String, Segment> {
    return lines.into_iter().map(|line| str_to_segment(line.as_str())).map(|s| (s.name.clone(), s)).collect();
}

pub fn lines_to_symbols(lines: &[String], module: &'static str) -> HashMap<String, Symbol> {
    return lines.into_iter().map( |line| str_to_symbol(line.as_str(), module) )
                            .map( |s| (s.name.clone(), s) ).collect();
}

pub fn do_common_blocks(symbols: &mut HashMap<String, Symbol>, segments: &mut HashMap<String, Segment>) {
    let undef_symbols_val = symbols.values().filter(|&sym| sym.sym_type == "U")
                                      .fold(0, |mut len, symb| {len += symb.value; len});
    segments.get_mut(BSS_SEG).unwrap().add_len(undef_symbols_val);
}

