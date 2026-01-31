use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

fn main() {
    let greeting_file_result = File::open("hello.txt");

    // panic on every errro
    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };

    // handle not found error and panic for others
    let greeting_file_result = File::open("hello.txt");
    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };

    // instead of using match on Result enum, can use unwrap that returns the value inside Ok
    // variant, or calls panic()
    let _greeting_file = File::open("hello.txt").unwrap();

    // another way is to call expect that return the value inside Ok (like unwrap), but excepts a
    // custom message for panic
    let _greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
}

// returns a result with generics as <String, io::Error>, the error will be only of type io::Error,
// therefore its better to specify the type of error
fn _read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    // assigns file from Ok to username_file var or returns Err from the function
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    // returns Ok with username or Err from the function
    match username_file.read_to_string(&mut username) {
        // ignores the value inside of Ok
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn _read_username_from_file_v2() -> Result<String, io::Error> {
    // the ? operator assigns the Ok variant value to the username_file var, or returns the Err for
    // the WHOLE function
    // the return type for the error depends on the error return type of the calling function
    // which is io::Error in this case
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn _read_username_from_file_v3() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// fs::read_to_string function opens the file, creates a new String, reads the contents of the file, puts the contents into that String, and returns it
fn _read_username_from_file_v4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
