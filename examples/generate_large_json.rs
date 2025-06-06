use fake::faker::{address::en::*, chrono::en::*, company::en::*, internet::en::*, name::en::*};
use fake::rand::prelude::IndexedRandom;
use fake::{Dummy, Fake, Faker, rand};
use json_writer::JSONArrayWriter;
use serde::Serialize;
use std::fs::File;
use std::io;
use std::io::Write;

#[derive(Debug, Serialize, Dummy)]
struct User {
    name: String,
    job: String,
    company: String,
    ssn: String,
    residence: String,
    current_location: (f64, f64),
    blood_group: String,
    website: String,
    username: String,
    sex: String,
    address: String,
    mail: String,
    birthdate: String,
}
impl User {
    fn new() -> Self {
        User {
            name: Name().fake(),
            job: Profession().fake(),
            company: CompanyName().fake(),
            ssn: Faker.fake::<String>(), // can be customized
            residence: SecondaryAddress().fake::<String>() + "\n" + &CityName().fake::<String>(),
            current_location: (Latitude().fake(), Longitude().fake()),
            blood_group: ["A+", "A-", "B+", "B-", "AB+", "AB-", "O+", "O-"]
                .choose(&mut rand::rng())
                .unwrap()
                .to_string(),
            website: FreeEmail().fake(),
            username: Username().fake(),
            sex: ["M", "F"].choose(&mut rand::rng()).unwrap().to_string(),
            address: StreetName().fake(),
            mail: SafeEmail().fake(),
            birthdate: Date().fake(),
        }
    }
}

fn main() {
    let mut input = String::new();
    let size_in_gb: f64;
    loop {
        input.clear();

        println!("Enter the size of the generated JSON file in GB:");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<f64>() {
            Ok(val) if val > 0.0 && val <= 5.0 => {
                size_in_gb = val;
                break;
            }
            _ => println!("Please enter a valid number between 0.0 and 5.0.\n"),
        }
    }

    let num_of_users = (size_in_gb * 2.80888e6) as usize;
    let path = format!("tests/data/users_{:.1}_gb.json", size_in_gb);
    println!("Writing users to {}", path);
    let mut file = File::create(path.as_str()).unwrap();
    let mut buffer = String::new();
    let mut array = JSONArrayWriter::new(&mut buffer);
    for _ in 0..num_of_users {
        let user = User::new();
        let mut user_obj = array.object();
        user_obj.value("name", &user.name);
        user_obj.value("job", &user.job);
        user_obj.value("company", &user.company);
        user_obj.value("ssn", &user.ssn);
        user_obj.value("residence", &user.residence);
        user_obj.value(
            "current_location",
            &vec![user.current_location.0, user.current_location.1],
        );
        user_obj.value("blood_group", &user.blood_group);
        user_obj.value("website", &user.website);
        user_obj.value("username", &user.username);
        user_obj.value("sex", &user.sex);
        user_obj.value("address", &user.address);
        user_obj.value("mail", &user.mail);
        user_obj.value("birthdate", &user.birthdate);
        user_obj.end();

        array.output_buffered_data(&mut file).unwrap();
        file.write_all(b"\n").unwrap();
    }

    array.end();
    file.write_all(buffer.as_bytes()).unwrap();
    println!(
        "Finished writing {}! Now you can run tests on this file",
        path
    );
}
