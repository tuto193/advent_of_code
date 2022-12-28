#![allow(dead_code)]
use std::{fs::File, io::Read, path::Path};

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;

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
        }
        Ok(_) => contents,
    }
}

fn main() {
    // let foo: usize = "012345".parse().unwrap();
    // println!("Number is: {}", foo);
    day_06::day_06_part2();
}
