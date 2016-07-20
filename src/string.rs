pub fn string() {
    println!("Testing string");
    let s1 = "Hello";
    let s2: &str = "World";
    println!("{}, {}!", s1, s2);

    let mut s3 = "Nihao";
    let mut s4 = "Shijie";
    println!("{}, {}!", s3, s4);

    s3 = "Hello";
    s4 = s2;
    println!("{}, {}!", s3, s4);

    // rustc --explain E0384
    // s1 = "Hello";
    // s2 = "World";
    // println!("{}, {}!", s1, s2);
}
