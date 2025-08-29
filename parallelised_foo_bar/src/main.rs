use std::thread;

fn main() {
    for i in 1..=100_000 {
        thread::spawn(move || match (i % 3, i % 5) {
            (0, 0) => println!("Foobar"),
            (0, _) => println!("Foo"),
            (_, 0) => println!("Bar"),
            (_, _) => println!("{i}"),
        });
    }
}
