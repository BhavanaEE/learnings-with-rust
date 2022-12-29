// Error handling
// shortcut for panic using unwrap and expect
use std::fs::File;

fn main() {

    //unwrap
    let f1 = File::open("hello.txt").unwrap();

    // expect
    let f2 = File::open("hello.txt")
        .expect("No file named hello.txt");
}
