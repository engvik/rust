type Kilometers = i32;
type Thunk = Box<dyn Fn() + Send + 'static>;

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    let _f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
    let _ff: Thunk = Box::new(|| println!("hi"));

    /*
    print!("forever ");

    loop {
        print!("and ever ");
    }
    */
}

fn _takes_long_type(_f: Box<dyn Fn() + Send + 'static>) {
    // -- snip --
}

fn _returns_long_type() -> Box<dyn Fn() + Send + 'static> {
    // -- snip --
    Box::new(|| println!("hi"))
}

fn _takes_long_type2(_f: Thunk) {
    // -- snip --
}

fn _returns_long_type2() -> Thunk {
    // -- snip --
    Box::new(|| println!("hi"))
}

/*
fn bar() -> ! {
    // -- snip --
}
*/
