// closure = anonymous function

fn main() {
    let num = 5;
    let add_num = |x| x + num;
    let result = add_num(3);
    println!("Result: {}", result); // prints 8
}
