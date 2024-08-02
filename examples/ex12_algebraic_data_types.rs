enum Color {
    Red,
    Green,
    Blue,
    RGB(u8, u8, u8),
}

fn main() {
    let colours = [Color::RGB(3, 4, 1), Color::Red, Color::Blue, Color::Green];
    for colour in colours {
        match colour {
            Color::Red => println!("The color is red"),
            Color::Green => println!("The color is green"),
            Color::Blue => println!("The color is blue"),
            Color::RGB(r, g, b) => println!("The color is ({}, {}, {})", r, g, b),
        }
    }
}
