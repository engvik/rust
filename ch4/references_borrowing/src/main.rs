fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    {
        let r1 = &mut s;
        println!("{}", r1);
    }

    let r2 = &mut s;
    println!("{}", r2);

    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);

    let no_dangle = no_dangle();
    println!("{}", no_dangle);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
