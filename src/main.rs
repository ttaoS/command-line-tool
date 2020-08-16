extern crate clap;
extern crate serde;

use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Read;

use clap::{App, load_yaml};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
}

pub trait Display {
    fn display(&self) -> String;
}

impl Display for User {
    fn display(&self) -> String {
        format!("name: {}, age: {}", self.name, self.age)
    }
}


fn print<'a>(vals: impl Iterator<Item = &'a User>){
    for user in vals {
        println!("{}", user.display());
    }
}

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();
    let file = matches.value_of("INPUT").unwrap();

    let mut file = File::open(file).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    // Convert the Point to a JSON string.
    let users: Vec<User> = serde_json::from_str(&data).unwrap();

    if let Some(string) = matches.value_of("count") {
        let number = string.parse().unwrap();
        let itr = users.iter().take(number);
        print(itr)
    } else {
        let itr = users.iter();
        print(itr)
    }
}
