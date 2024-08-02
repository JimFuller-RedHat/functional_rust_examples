trait Dispatcher {
    fn dispatch(&self, msg: &str);
}

struct Logger;

impl Dispatcher for Logger {
    fn dispatch(&self, msg: &str) {
        println!("Logging: {}", msg);
    }
}

struct Printer;

impl Dispatcher for Printer {
    fn dispatch(&self, msg: &str) {
        println!("Printing: {}", msg);
    }
}

fn main() {
    let logger = Logger;
    let printer = Printer;

    let dispatchers: Vec<&dyn Dispatcher> = vec![&logger, &printer];

    for dispatcher in dispatchers {
        dispatcher.dispatch("Hello, world!");
    }
}
