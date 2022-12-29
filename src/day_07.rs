use std::{
    // borrow::BorrowMut,
    collections::HashMap,
    // ops::Deref,
    // ops::{Deref, DerefMut},
    // rc::Rc,
};

use crate::get_file_contents;

#[derive(Clone, Debug)]
struct Directory {
    name: String,
    // parent: Option<Box<Directory>>,
    dirs: HashMap<String, Directory>,
    files: Vec<usize>,
}

impl Directory {
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            // parent: match parent {
            //     None => None,
            //     Some(parent) => Some(Box::new(*parent)),
            // },
            dirs: HashMap::new(),
            files: vec![],
        }
    }

    pub fn total_size(&self) -> usize {
        let mut sum = 0;
        for f in self.files.iter() {
            sum += *f;
        }
        for d in self.dirs.iter() {
            // let the_dir = dir_list.get(d).unwrap();
            sum += d.1.total_size();
        }
        sum
    }

    // pub fn set_parent(mut self, p: Directory) {
    //     match self.parent {
    //         Some(n) => println!("Tried to set parent from {} to {}", n, p),
    //         None => self.parent = Some(p.to_string()),
    //     }
    // }

    pub fn add_file(&mut self, file: usize) {
        self.files.push(file)
    }

    pub fn add_dir(&mut self, dir: &str) {
        // let my_clone = self.clone();
        // let my_self_rc = Rc::new(self);
        // let to_add = Directory::new(dir.to_string(), Some(&self.name));
        let to_add = Directory::new(dir.to_string());
        self.dirs.insert(to_add.name.clone(), to_add);
    }

    // pub fn get_parent(&self) -> &Box<Directory> {
    //     // never use `..` on `/`
    //     self.parent.as_ref().unwrap()
    //     // to_show
    // }

    pub fn get_files(self) -> Vec<usize> {
        self.files
    }

    pub fn get_dirs(&mut self) -> &mut HashMap<String, Directory> {
        &mut self.dirs
    }
}

fn get_directories_from_input(input: String) -> HashMap<String, Directory> {
    let mut dirs: HashMap<String, Directory> = HashMap::new();
    let commands = input.split("$ ").collect::<Vec<&str>>();
    let mut active_dirs: Vec<String> = vec![];
    for command in commands.into_iter().skip(1) {
        // println!("Command looks like ' {} '.", command);
        let lines = command.split("\n").collect::<Vec<&str>>();
        // println!("Lines after split are now: '{:?}'", lines);
        if lines.len() == 2 {
            // It is a `cd` command = new directory
            let cd_command_line = lines[0].split(" ").collect::<Vec<&str>>();
            // println!("Command line was length one and looks like: {:?}", cd_command_line);
            match cd_command_line[1] {
                ".." => {
                    // go back to parent dir
                    // If this ever panics, it's because we tried to `..` at `/`
                    active_dirs.pop();
                }
                dir_name => {
                    // get into a new sub-directory (we found it through `ls`)
                    // println!("Directory name is '{}'", dir_name);
                    let new_dir = Directory::new(dir_name.to_string());
                    // let already_there =
                    active_dirs.push(dir_name.to_string());
                    let _already_there = dirs.insert(dir_name.to_string(), new_dir);
                }
            }
        } else {
            // it's an `ls` command = we need to split check
            // Skip the `ls` part
            for line in lines.into_iter().skip(1) {
                let output = line.split(" ").collect::<Vec<&str>>();
                let active_dir = dirs.get_mut(active_dirs.last().unwrap()).unwrap();
                match output[0] {
                    "dir" => {
                        // let cur_dir = mut_active_dir.clone();
                        let found_dir_name = output[1];
                        // let to_add_dir = Directory::new(found_dir_name.to_string(), Some(mut_active_dir));
                        // let active_dir = active_dirs.last().unwrap();
                        active_dir.add_dir(found_dir_name);
                    }
                    "" => continue,
                    f_size => {
                        println!("File '{}'\t is '{}'", output[1], f_size);
                        let file_size: usize = f_size.parse().unwrap();
                        // let mut actual_mut_active_dir = mut_active_dir.deref();
                        // let active_dir = active_dirs.last().unwrap();
                        active_dir.add_file(file_size);
                    }
                }
            }
        }
    }
    dirs
}

pub fn day_07() {
    // let input = get_file_contents("07".to_string());
    let input = get_file_contents("007".to_string());
    let directories = get_directories_from_input(input);
    let max_size: usize = 100000;
    let mut total_sum: usize = 0;
    let directories_filtered = directories.into_iter().filter(|nd| {
        println!("Dir {} contains {:?}", nd.1.name, nd.1.files);
        println!("Dir {} total size {}", nd.1.name, nd.1.total_size());
        nd.1.total_size() <= max_size
    });
    for (n, d) in directories_filtered {
        total_sum += d.total_size();
    }
    println!("The total sum of all under {} is: {}", max_size, total_sum)
}
