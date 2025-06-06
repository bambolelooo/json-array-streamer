use std::fs::File;
use std::{env, io};

pub fn ask_filename_and_open() -> File {
    loop {
        println!("what is the name of the file?");
        println!("example: users_1.0_gb.json");
        let mut file_name = String::new();
        io::stdin()
            .read_line(&mut file_name)
            .expect("Failed to read line");
        match File::open(format!("examples/{file_name}")) {
            Ok(f) => {
                println!("OK!");
                return f;
            }
            Err(e) => {
                println!("Error opening file: {}", e);
                println!("Please enter a valid file name.");
            }
        }
    }
}

pub fn load_from_env_or_ask() -> File {
    let file: File;

    match env::args().nth(1) {
        Some(path) => match File::open(format!("examples/{}", path.as_str())) {
            Ok(res) => file = res,
            Err(_) => {
                println!("Could not open {path:?}");
                file = ask_filename_and_open()
            }
        },
        None => {
            file = ask_filename_and_open();
        }
    }
    file
}
