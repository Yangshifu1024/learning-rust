#![feature(inclusive_range_syntax)]
pub fn main() {
    for i in 0...10 {
        println!("{}", i);
    }
    println!("----");
    for i in [0, 1, 2].iter() {
        println!("{}", i);
    }
}
