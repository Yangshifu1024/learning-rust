pub fn boxes() {
    let boxed = Some(box 5);

    match boxed {
        Some(box unboxed) => println!("Some {}", unboxed),
        None => println!("None"),
    }
}
