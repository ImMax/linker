pub mod parser;
pub mod obj_file;
pub mod symbol;
pub mod segment;

use obj_file::ObjectFile;

fn main() {
    let obj1 = ObjectFile::from_file("obj1.txt");
    println!("{}", obj1);
    let obj2 = ObjectFile::from_file("obj2.txt");
    println!("{}", obj2);
    let obj3 = ObjectFile::combine(vec![obj1, obj2]);
    println!("==================================");
    println!("{}", obj3);
}
