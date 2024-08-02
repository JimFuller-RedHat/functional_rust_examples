fn countdown(c: i32) -> Vec<i32> {
    if c == 0 {
        vec![]
    } else {
        let mut result = vec![c];
        result.extend(countdown(c - 1));
        result
    }
}

fn main() {
    let result = countdown(20);
    println!("{:?}", result); // prints [20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]
}
