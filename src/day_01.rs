
pub fn day_01() -> Option<usize> {
    let bind = crate::get_file_contents("01".into());
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
