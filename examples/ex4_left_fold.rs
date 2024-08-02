fn fold_left<I, T, U>(iter: I, init: U, f: impl Fn(U, T) -> U) -> U
where
    I: Iterator<Item = T>,
{
    iter.fold(init, f)
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let result = fold_left(numbers.iter(), 0, |acc, x| acc + x);
    println!("Result: {}", result); // prints 15
}
