use std::collections::HashMap;

struct Memoizer {
    cache: HashMap<u32, u32>,
}

impl Memoizer {
    fn new() -> Memoizer {
        Memoizer {
            cache: HashMap::new(),
        }
    }

    fn factorial(&mut self, n: u32) -> u32 {
        if let Some(&result) = self.cache.get(&n) {
            return result;
        }

        let result = match n {
            0 => 1,
            _ => n * self.factorial(n - 1),
        };

        self.cache.insert(n, result);
        result
    }
}

fn main() {
    let mut memoizer = Memoizer::new();

    println!("Factorial of 1: {}", memoizer.factorial(1));
    println!("Factorial of 3: {}", memoizer.factorial(3));
    println!("Factorial of 7: {}", memoizer.factorial(7));
    println!("Factorial of 11: {}", memoizer.factorial(11));
}
