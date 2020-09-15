fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let cond = true;
    let number = if cond { 5 } else { 6 };
    println!("The value of number is: {}", number);

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {}", result);

    let mut num = 3;

    while num != 0 {
        println!("{}", num);
        num -= 1;
    }

    println!("LIFTOFF!!!");

    let arr = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", arr[index]);

        index += 1;
    }

    for element in arr.iter() {
        println!("The value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}", number);
    }

    println!("LIFTOFF!!!");
}
