pub fn for_loop() {
    println!("Testing for loop");
    for i in 0...10 {
        println!("{}", i);
    }
    println!("////////////");
    for i in [0, 1, 2].iter() {
        println!("{}", i);
    }
}
