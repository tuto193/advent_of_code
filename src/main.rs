use std::{path::Path, fs::File, io::Read};

mod day_01;
mod day_02;

pub fn get_file_contents(day: String) -> String {
    let input_path = Path::new(&format!("inputs/{}.txt", day));
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
    println!("The elf is carrying: {}", day_01::day_01().unwrap());
}
