fn function_without_return(x: i32) {
    println!("x is {}", x);
}

fn function_with_return(x: i32) -> i32 {
    x * x
}

fn function_with_ref(x: &mut i32) {
    *x *= 2
}

pub fn main() {
    function_without_return(2);
    println!("//////////////");
    let ret: i32 = function_with_return(4);
    println!("4 * 4 = {}", ret);
    println!("//////////////");
    let mut four: i32 = 4;
    function_with_ref(&mut four);
    println!("4 * 2 = {}", four);
}
