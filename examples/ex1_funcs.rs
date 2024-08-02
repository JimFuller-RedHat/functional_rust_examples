fn add_x(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

fn main() {
    let add_three = add_x(3);
    let result = add_three(5);
    println!("Result: {}", result); // prints 8
}
