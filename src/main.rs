use std::collections::LinkedList;
use std::thread::Thread;

fn test(a: i32, b: i32) -> i32 {
    let e = 6;
    println!("{}", e);
    return a + b;
}

fn main() {
    let x = 5;
    let y = 2;
    test(x, y);

}
