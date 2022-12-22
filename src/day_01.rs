use std::{path::Path, fs::File, io::Read};

pub fn day_01() -> Option<usize> {
    let bind = get_elves_vector();
    let elves_vector: Vec<&str> = bind.split("\n\n").collect();

    let mut max = 0;
    for elf in elves_vector.into_iter() {
        let elf_string: Vec<&str> = elf.split("\n").collect();
        let mut sum = 0;
        for line in elf_string.into_iter() {
            if line != "" {
                sum += line.parse::<usize>().unwrap();
            }
        }
        if sum > max {
            max = sum;
        }
    }
    Some(max)
}

fn get_elves_vector() -> String {
    let input_path = Path::new("inputs/01.txt");
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
