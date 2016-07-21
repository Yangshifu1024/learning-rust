#[derive(Debug)]
struct Number {
    val: i32,
}

trait Sub {
    fn sub(&self, Number) -> i32;
}

trait Add {
    fn add(&self, Number) -> i32;
}

impl Sub for Number {
    fn sub(&self, another: Number) -> i32 {
        return self.val - another.val;
    }
}

impl Add for Number {
    fn add(&self, another: Number) -> i32 {
        return self.val + another.val;
    }
}

pub fn main() {
    let x: Number = Number { val: 123 };
    let y: Number = Number { val: 456 };
    let ret: i32 = x.add(y);
    println!("{:?}", ret);
}
