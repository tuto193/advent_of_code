#![allow(dead_code)]
use std::{path::Path, fs::File, io::Read};

mod day_01;
mod day_02;
mod day_03;
mod day_04;

pub fn get_file_contents(day: String) -> String {
    let filename = format!("inputs/{}.txt", day);
    let input_path = Path::new(filename.as_str());
    let mut file = match File::open(&input_path) {
        Err(why) => panic!("Couldn't open {}: {}", input_path.display(), why),
        Ok(file) => file,
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(why) => {
            panic!("Couldn't read {}: {}", input_path.display(), why);
        },
        Ok(_) => {
            contents
        },
    }
}

fn main() {
    // let foo: usize = "012345".parse().unwrap();
    // println!("Number is: {}", foo);
    day_04::day_04();
}
