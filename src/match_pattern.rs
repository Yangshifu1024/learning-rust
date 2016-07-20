#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

fn match_pair(pair: (f64, f64)) {
    match pair {
        (1.2, y) => println!("x is 1.2, y is {}", y),
        (x, 4.3) => println!("x is {}, y is 4.3", x),
        _ => println!("Doesn't match anything"),
    }
}

fn match_point(point: Point) {
    match point {
        Point { x: 1, y } => println!("({:?}).y {:?}", point, y),
        Point { x, .. } => println!("({:?}).x {:?}", point, x),
    }
}

pub fn match_pattern() {
    println!("Testing match");
    let day: u8 = 5;
    match day {
        0 | 6 => println!("weekend!"),
        d @ 1...5 => println!("weekday {}", d),
        _ => println!("invalid"),
    }
    println!("//////////");

    let pair1: (f64, f64) = (1.2, 3.4);
    match_pair(pair1);
    let pair2: (f64, f64) = (2.1, 4.3);
    match_pair(pair2);
    let pair3: (f64, f64) = (5.6, 7.8);
    match_pair(pair3);
    println!("//////////");

    let p1 = Point { x: 1, y: 2 };
    match_point(p1);
    let p2 = Point { x: 3, y: 4 };
    match_point(p2);
    println!("//////////");

    let o: Option<i32> = Some(99i32);
    if let Some(i) = o {
        println!("{}", i);
    } else {
        println!("None");
    }

    let o1: Option<i32> = None;
    if let Some(i) = o1 {
        println!("{}", i);
    } else {
        println!("None");
    }
}
