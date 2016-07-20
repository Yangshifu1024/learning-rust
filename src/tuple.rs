pub fn tuple() {
    println!("Testing tuples");

    let tuple: (&str, &str) = ("Hello", "World");
    println!("tuple: {0:?}", tuple);

    let (hello, _) = tuple;
    println!("hello: {}", hello);

    let world = tuple.1;
    println!("world: {}", world);
}
