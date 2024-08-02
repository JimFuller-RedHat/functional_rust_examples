fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let add_three = |y| add(3, y);
    let result = add_three(5);
    println!("Result: {}", result); // prints 8
}
