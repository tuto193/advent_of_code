use std::{
    collections::HashMap,
    // ops::{Deref, DerefMut},
    rc::Rc, ops::Deref, borrow::BorrowMut,
};

use crate::get_file_contents;

#[derive(Clone, Debug)]
struct Directory {
    name: String,
    parent: Option<String>,
    dirs: Vec<String>,
    files: Vec<usize>,
}

impl Directory {
    pub fn new(name: String, parent: Option<String>) -> Self {
        Self {
            name: name,
            parent: match parent {
                Some(p) => {Some(p)}
                None => None,
            },
            dirs: vec![],
            files: vec![],
        }
    }

    pub fn total_size(&self, dir_list: &HashMap<String, Rc<&mut Directory>>) -> usize {
        let mut sum = 0;
        for f in self.files.iter() {
            sum += f;
        }
        for d in self.dirs.iter() {
            let the_dir = dir_list.get(d).unwrap();
            sum += the_dir.total_size(dir_list);
        }
        sum
    }

    pub fn set_parent(& mut self, p: &str) {
        match &self.parent {
            Some(n) => println!("Tried to set parent from {} to {}", n, p),
            None => self.parent = Some(p.to_string()),
        }
    }

    pub fn add_file(&mut self, file: usize) {
        self.files.push(file)
    }

    pub fn add_dir(& mut self, dir: &str) {
        // let my_clone = self.clone();
        // let my_self_rc = Rc::new(self);
        // let to_add = Directory::new(dir.to_string(), Some(&self.name));
        &self.dirs.push(dir.to_string());
    }

    pub fn get_parent<'a>(&'a self, dir_list: &HashMap<String, Rc<&'a mut Directory>>) -> Option<Rc<&mut Directory>> {
        match &self.parent {
            Some(p) => {
                let parent_dir = dir_list.get(p).unwrap();
                Some(parent_dir.clone())
            }
            None => {
                println!("I has no parents");
                None
            }
        }
    }

    pub fn get_files(&self) -> &Vec<usize> {
        &self.files
    }

    pub fn get_dirs(&mut self) -> &mut Vec<String> {
        &mut self.dirs
    }
}

pub fn day_07() {
    let input = get_file_contents("07".to_string());
    let mut dirs: HashMap<String, Rc<&mut Directory>> = HashMap::new();
    let commands = input.split("$ ").collect::<Vec<&str>>();
    let mut root_dir = Directory::new("/".to_string(), None);
    let mut active_dir = Rc::new(&mut root_dir);
    // let mut last_active_dir: Option<Directory> = None;
    dirs.insert(active_dir.name.clone(), Rc::clone(&active_dir));
    for command in commands.into_iter().skip(1) {
        let lines = command.split("\n").collect::<Vec<&str>>();
        if lines.len() == 1 {
            // It is a `cd` command = new directory
            let cd_command_line = lines[0].split(" ").collect::<Vec<&str>>();
            match cd_command_line[1] {
                ".." => {
                    // go back to parent dir
                    // If this ever panics, it's because we tried to `..` at `/`
                    let parent = active_dir.get_parent(&dirs);
                    let parent = parent.unwrap();
                    active_dir = parent;
                }
                dir_name => {
                    // get into a new sub-directory (we found it through `ls`)
                    match dirs.get(dir_name) {
                        Some(d) => {
                            active_dir = Rc::clone(d);
                        }
                        None => {
                            let new_dir = Directory::new(dir_name.to_string(), Some(active_dir.name));
                        }
                    }
                }
            }
        } else {
            // it's an `ls` command = we need to split check
            for line in lines.into_iter() {
                let output = line.split(" ").collect::<Vec<&str>>();
                match output[0] {
                    "dir" => {
                        // let cur_dir = active_dir.clone();
                        let found_dir_name = output[1];
                        active_dir.add_dir(found_dir_name);
                    }
                    f_size => {
                        let file_size: usize = f_size.parse().unwrap();
                        // let mut actual_active_dir = active_dir.deref();
                        active_dir.add_file(file_size);
                    }
                }
            }
        }
    }
}
