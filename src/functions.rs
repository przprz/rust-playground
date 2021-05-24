#[derive(Debug)]
struct Foo;

struct Bar;

impl Foo {
    fn bar(&self) {
        println!("hi");
    }
}

impl Bar {
    fn bar(&self) {
        println!("hello");
    }
}

pub fn f() {
    Foo.bar(); // prints hi
    Bar.bar(); // prints hello

    let b = Bar {};
    b.bar();
}
