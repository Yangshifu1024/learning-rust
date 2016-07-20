pub fn destruction() {
    println!("Testing destruction");
    let (a, b, c) = (0, 1, 123_456);
    let (d, e): (f64, f64) = (4.56e-2, 4.56e+2);
    println!("a:{} b:{} c:{}", a, b, c);
    println!("d:{} e:{}", d, e);
}
