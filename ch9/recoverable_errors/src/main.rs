use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    /*
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening file: {:?}", error),
    };
    */

    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    let _f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // let _f = File::open("hello2.txt").unwrap();
    // let _f = File::open("hello2.txt").expect("Failed to open hello2.txt");

    let username = read_username_from_file();
    let _username = match username {
        Ok(username) => println!("username: {}", username),
        Err(e) => panic!("error reading username: {:?}", e),
    };

    let username = read_username_from_file_short();
    let _username = match username {
        Ok(username) => println!("username: {}", username),
        Err(e) => panic!("error reading username: {:?}", e),
    };

    let username = read_username_from_file_shorter();
    let _username = match username {
        Ok(username) => println!("username: {}", username),
        Err(e) => panic!("error reading username: {:?}", e),
    };

    let username = read_username_from_file_shortest();
    let _username = match username {
        Ok(username) => println!("username: {}", username),
        Err(e) => panic!("error reading username: {:?}", e),
    };
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("username.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut f = File::open("username.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("username.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn read_username_from_file_shortest() -> Result<String, io::Error> {
    fs::read_to_string("username.txt")
}

/*
fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}
*/
