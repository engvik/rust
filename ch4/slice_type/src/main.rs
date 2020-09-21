fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("hello, world!");

    let w1 = first_word(&s1);
    let w2 = first_word(&s2);
    println!("w1: {}, w2: {}", w1, w2);

    let hello = &s2[0..5];
    let world = &s2[7..12];
    println!("hello: {}, world: {}", hello, world);

    let sl1 = &s1[0..2];
    let sl2 = &s1[..2];
    println!("sl1: {}, sl2: {}", sl1, sl2);

    let len = s1.len();
    let sl1 = &s1[3..len];
    let sl2 = &s1[3..];
    println!("sl1: {}, sl2: {}", sl1, sl2);

    let sl1 = &s1[0..len];
    let sl2 = &s1[..];
    println!("sl1: {}, sl2: {}", sl1, sl2);

    let w1 = first_word2(&s1);
    let w2 = first_word2(&s2);
    println!("w1: {}, w2: {}", w1, w2);

    let my_string = String::from("hello world!");
    let my_string_literal = "hello world!";
    let w1 = first_word3(&my_string[..]);
    let w2 = first_word3(&my_string_literal[..]);
    let w3 = first_word3(my_string_literal);
    println!("w1: {}, w2: {}, w3: {}", w1, w2, w3);

    let a = [1, 2, 3, 4, 5];
    let _slice = &a[1..3];
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word3(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
