use std::collections::HashMap;

use crate::get_file_contents;

struct File {
    size: usize,
    // name: String,
}

struct Directory {
    name: String,
    parent: Option<Box<Directory>>,
    dirs: HashMap<String, Directory>,
    files: Vec<File>,
}

impl Directory {
    pub fn new(name: String, parent: Option<Directory>) -> Self {
        Self {
            name: name,
            parent: match parent {
                Some(p) => Some(Box::new(p)),
                None => None,
            },
            dirs: HashMap::new(),
            files: vec![],
        }
    }

    pub fn total_size(&self) -> usize {
        let mut sum = 0;
        for f in self.show_files().into_iter() {
            sum += &f.size;
        }
        for d in self.show_dirs().into_iter() {
            sum += d.1.total_size();
        }
        sum
    }

    pub fn set_parent(&mut self, p: Directory) {
        match &self.parent {
            Some(n) => println!("Tried to set parent from {} to {}", n.name, p.name),
            None => self.parent = Some(Box::new(p)),
        }
    }

    pub fn add_file(&mut self, file: File) {
        self.files.push(file)
    }

    pub fn add_dir(mut self, dir: Directory) {
        self.dirs.insert(dir.name.clone(), dir);
    }

    pub fn show_parent(self) -> Option<Directory> {
        match self.parent {
            Some(p) => Some(*p),
            None => {
                println!("I has no parents");
                None
            }
        }
    }

    pub fn show_files(&self) -> &Vec<File> {
        &self.files
    }

    pub fn show_dirs(&self) -> &HashMap<String, Directory> {
        &self.dirs
    }
}

impl File {
    pub fn new(size: usize) -> Self {
        Self { size: size }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }
}

pub fn day_07() {
    let input = get_file_contents("07".to_string());
    let mut dirs: HashMap<String, &Directory> = HashMap::new();
    let commands = input.split("$ ").collect::<Vec<&str>>();
    let mut active_dir = Directory::new("/".to_string(), None);
    let mut last_active_dir: Option<&Directory> = None;
    dirs.insert(active_dir.name.clone(), &active_dir);
    for command in commands.into_iter().skip(1) {
        let lines = command.split("\n").collect::<Vec<&str>>();
        if lines.len() == 1 {
            // It is a `cd` command
            let cd_command_line = lines[0].split(" ").collect::<Vec<&str>>();
            let dir_name = cd_command_line[1];
            let dir = Directory::new(dir_name.to_string(), last_active_dir)
        }
    }
}
