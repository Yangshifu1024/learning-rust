#![feature(inclusive_range_syntax)]
pub fn main() {
    let arr1: [i32; 3] = [0; 3];
    println!("arr1: {:?}", arr1);
    let arr2: [i32; 3] = [1, 2, 3];
    println!("arr2: {:?} arr2[1..]: {:?} arr2[1..2]: {:?} arr2[1...2]: {:?}",
             arr2,
             &arr2[1..],
             &arr2[1..2],
             &arr2[1...2]);
}
