use std::thread;
use std::sync::Arc;

#[derive(Debug)]
struct Thhh;

pub(crate) fn t() {
    let f = Thhh;
    let f1 = Arc::new(f);
    let f2 = f1.clone();

    let t1 = thread::spawn(move || { println!("{:?}", f1) });
    let t2 = thread::spawn(move || { println!("{:?}", f2) });
    t1.join().unwrap();
    t2.join().unwrap();
}

