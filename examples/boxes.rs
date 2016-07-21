#![feature(box_syntax, box_patterns)]
pub fn main() {
    let boxed = Some(box 5);

    match boxed {
        Some(box unboxed) => println!("Some {}", unboxed),
        None => println!("None"),
    }
}
