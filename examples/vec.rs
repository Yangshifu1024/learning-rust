pub fn main() {
    let v1 = vec![1, 2, 3];
    let v2: Vec<i32> = vec![4, 5, 6];
    println!("v1: {:?} v2: {:?}", v1, v2);

    let mut v3: Vec<f64> = vec![7.1, 8.2, 9.3];
    println!("v3: {:?}", v3);

    println!("Push 10.4 to v3");
    v3.push(10.4);
    println!("v3: {:?}", v3);

    let mut v4 = v3;
    println!("Push 11.5 to v4");
    v4.push(11.5);
    println!("v4: {:?}", v4);
    println!("Reset v3 to []");
    println!("v3 here is not v3 above");
    v3 = vec![];
    println!("v3: {:?} v4: {:?}", v3, v4);

    {
        let mut v5 = &mut v4;
        println!("Pop from v5");
        v5.pop();
        println!("v5: {:?}", v5);
    }
    println!("v4 has been changed by its ref v5");
    println!("v4: {:?}", v4);
}
