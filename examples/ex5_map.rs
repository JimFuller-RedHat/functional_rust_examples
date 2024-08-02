// use map to apply anon func
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let squares = numbers.iter().map(|x| x * x).collect::<Vec<_>>();
    println!("{:?}", squares); // prints [1, 4, 9, 16, 25]
}
