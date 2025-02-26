use std::thread;

fn main() {
    let child = thread::Builder::new().stack_size(32 * 1024 * 1024).spawn(move || {
        println!("hello world");
        "world"
    }).unwrap();

    let v = child.join().unwrap();

    println!("child result: {}", v);
}