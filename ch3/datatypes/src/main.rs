fn main() {
    let num: u32 = "42".parse().expect("Not a number!");
    println!("Num: {}", num);
    let num: u32 = 0xff;
    println!("Num: {}", num);
    let num: u32 = 0o77;
    println!("Num: {}", num);
    let num: u32 = num + 0b1111_0000;
    println!("Num: {}", num);

    let num: f64 = 0.4;
    println!("Num: {}", num);

    let sum = 5 + 10;
    println!("Sum: {}", sum);

    let difference = 95.5 - 4.3;
    println!("Difference: {}", difference);

    let product = 4 * 30;
    println!("Product: {}", product);

    let quotient = 56.7 / 32.2;
    println!("Quotient: {}", quotient);

    let remainder = 43 % 5;
    println!("Remainder: {}", remainder);

    let t = true;
    let f: bool = false;
    println!("True: {} / False: {}", t, f);

    let mut c = 'c';
    println!("Char: {}", c);
    c = 'ℤ';
    println!("Char: {}", c);
    c = '😻';
    println!("Char: {}", c);

    let tup: (i32, f64, u8) = (-500, 6.4, 1);
    let (x, y, z) = tup;
    println!("X: {}, Y: {}, Z: {}", x, y, z);

    let tup = (-256, 6.2, 11);
    let (x, y, z) = tup;
    println!("X: {}, Y: {}, Z: {}", x, y, z);
    println!("X: {}, Y: {}, Z: {}", tup.0, tup.1, tup.2);

    let arr = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let arr_with_type: [i32; 5] = [1, 2, 3, 4, 5];
    let arr_init_values = [3; 5];
    println!("Arr (0): {}, Arr (3): {}", arr[0], arr[3]);
    println!("Months (0): {}, Months (3): {}", months[0], months[3]);
    println!("ArrWithType (0): {}, ArrWithType (3): {}", arr_with_type[0], arr_with_type[3]);
    println!("ArrInitValues (0): {}, ArrInitValues (3): {}", arr_init_values[0], arr_init_values[3]);
}
