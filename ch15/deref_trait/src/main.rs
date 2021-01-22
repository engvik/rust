use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

struct MyBoxDeref<T>(T);

impl<T> MyBoxDeref<T> {
    fn new(x: T) -> MyBoxDeref<T> {
        MyBoxDeref(x)
    }
}

impl<T> Deref for MyBoxDeref<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let _y = MyBox::new(x);

    assert_eq!(5, x);
    // assert_eq!(5, *y);

    let x = 5;
    let y = MyBoxDeref::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBoxDeref::new(String::from("Rust"));
    hello(&m);

    let m = MyBoxDeref::new(String::from("Rust"));
    hello(&(*m)[..]);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
