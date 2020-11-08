fn main() {
    let mut _s = String::new();

    let data = "initial contents";
    let s = data.to_string();
    println!("{}", s);

    let s = "initial contents".to_string();
    println!("{}", s);

    let s = String::from("initial contents");
    println!("{}", s);

    let hello = String::from("السلام عليكم");
    println!("{}", hello);

    let hello = String::from("Dobrý den");
    println!("{}", hello);

    let hello = String::from("Hello");
    println!("{}", hello);

    let hello = String::from("שָׁלוֹם");
    println!("{}", hello);

    let hello = String::from("नमस्ते");
    println!("{}", hello);

    let hello = String::from("こんにちは");
    println!("{}", hello);

    let hello = String::from("안녕하세요");
    println!("{}", hello);

    let hello = String::from("你好");
    println!("{}", hello);

    let hello = String::from("Olá");
    println!("{}", hello);

    let hello = String::from("Здравствуйте");
    println!("{}", hello);

    let hello = String::from("Hola");
    println!("{}", hello);

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);

    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2);
    println!("s {}", s);
    println!("s2 {}", s2);

    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    let _s1 = String::from("hello");
    // let h = s1[0];

    let hello = String::from("Здравствуйте");
    // let answer = &hello[0];
    let s = &hello[0..4];
    println!("{}", s);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
