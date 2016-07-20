use std::fmt;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct Length(f64);

impl fmt::Debug for Length {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Length(float 64): {} (Output by custom fmt method)",
               self.0)
    }
}

pub fn structs() {
    let p = Point { x: 0, y: 1 };
    println!("p: {:?}", p);

    let mut p1 = Point { x: 1, y: 2 };
    println!("p1: {:?}", p1);
    p1.x = 3;
    p1.y = 4;
    println!("p1: {:?}", p1);


    let l1 = Length(1.2345);
    println!("l1: {:?}", l1);
}
