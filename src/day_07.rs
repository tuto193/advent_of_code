use std::{collections::HashMap, ops::Index};

use crate::get_file_contents;

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: usize,
}
#[derive(Clone, Debug)]
struct Directory {
    name: String,
    // parent: Option<Box<Directory>>,
    dirs: HashMap<String, Directory>,
    files: Vec<File>,
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

    pub fn total_size(
        &self,
        // updated_dirs: &HashMap<String, Directory>
    ) -> usize {
        let mut sum = 0;
        for f in self.get_files().into_iter() {
            sum += f.size;
        }
        // for d in s.iter() {
        //     // let the_dir = dir_list.get(d).unwrap();
        // }
        // let my_dirs = updated_dirs
        //     .iter()
        //     .filter(|(n, &_)| self.dirs.contains_key(n.clone()))
        //     .collect::<HashMap<&String, &Directory>>();
        // for (_, d) in my_dirs.iter() {
        // for (_n, d) in self.dirs.iter() {
        //     sum += d.total_size();
        // }
        sum
    }

    // pub fn set_parent(mut self, p: Directory) {
    //     match self.parent {
    //         Some(n) => println!("Tried to set parent from {} to {}", n, p),
    //         None => self.parent = Some(p.to_string()),
    //     }
    // }

    pub fn add_file(&mut self, size: usize, name: String) {
        let to_add = File {
            name: name,
            size: size,
        };
        self.files.push(to_add)
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

    /// Returns a vector of all files in this and all its subdirectories
    pub fn get_files(&self) -> Vec<File> {
        let mut to_return = self.files.clone();
        for (_, dir) in self.dirs.iter() {
            // println!("Dir '{}' is in {}", n, self.name);
            let mut subdir_files = dir.get_files().into_iter().collect::<Vec<File>>();
            to_return.append(&mut subdir_files);
        }
        to_return
    }

    pub fn get_dirs(&mut self) -> &mut HashMap<String, Directory> {
        &mut self.dirs
    }

    pub fn get_all_under_limit(&self, max_limit: usize) -> usize {
        let mut total_sum = 0;
        let this_dirs_size = self.total_size();
        if this_dirs_size <= max_limit {
            total_sum += this_dirs_size;
        }
        for (_n, d) in self.dirs.iter() {
            total_sum += d.get_all_under_limit(max_limit);
        }
        total_sum
    }
}

fn get_directories_from_input(input: String) -> Vec<Directory> {
    let mut dirs: Vec<Directory> = vec![];
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
                    let sub_dir_name = active_dirs.pop().unwrap();
                    let sub_dir: Directory = dirs.pop().unwrap();
                    // Update entry in Parent's HashMap
                    let cur_dir: &mut Directory = dirs.last_mut().unwrap();
                    cur_dir.get_dirs().remove(&sub_dir_name);
                    cur_dir.get_dirs().insert(sub_dir_name.clone(), sub_dir);
                }
                dir_name => {
                    // get into a new sub-directory (we found it through `ls`)
                    // println!("Directory name is '{}'", dir_name);
                    let new_dir = Directory::new(dir_name.to_string());
                    // let already_there =
                    active_dirs.push(dir_name.to_string());
                    // let _already_there = dirs.insert(dir_name.to_string(), new_dir);
                    dirs.push(new_dir);
                }
            }
        } else {
            // it's an `ls` command = we need to split check
            // Skip the `ls` part
            for line in lines.into_iter().skip(1) {
                let output = line.split(" ").collect::<Vec<&str>>();
                let active_dir= dirs.last_mut().unwrap();
                match output[0] {
                    "dir" => {
                        // let cur_dir = mut_active_dir.clone();
                        let found_dir_name = output[1];
                        // let to_add_dir = Directory::new(found_dir_name.to_string(), Some(mut_active_dir));
                        // let active_dir = active_dirs.last().unwrap();
                        active_dir.get_dirs().insert(found_dir_name.to_string(), Directory::new(found_dir_name.to_string()));
                        // println!("Dir '{}': added dir '{}'", active_dir.name, output[1]);
                    }
                    "" => continue,
                    f_size => {
                        // println!("File '{}'\t is '{}'", output[1], f_size);
                        let file_size: usize = f_size.parse().unwrap();
                        // let mut actual_mut_active_dir = mut_active_dir.deref();
                        // let active_dir = active_dirs.last().unwrap();
                        active_dir.add_file(file_size, output[1].to_string());
                        // println!("Dir '{}': added file '{}'", active_dir.name, output[1]);
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
    let root = directories.index(0);
    let total_sum = root.get_all_under_limit(max_size);
    // let directories_wanted = directories.iter().filter(|(_, d)| {
    //     // println!("Dir {} contains {:?}", nd.1.name, nd.1.get_files());
    //     // println!("Dir {} total size {}", nd.1.name, nd.1.total_size());
    //     d.total_size(&directories) <= max_size
    // });
    // let directories_unwanted = directories.iter().filter(|(_, d)| {
    //     // println!("Dir {} contains {:?}", nd.1.name, nd.1.get_files());
    //     // println!("Dir {} total size {}", nd.1.name, nd.1.total_size());
    //     d.total_size(&directories) > max_size
    // });
    // for (n, d) in directories_unwanted {
    //     println!("Dir '{}' \t has total size '{}'", n, d.total_size(&directories));
    // }
    // for (_, d) in directories_wanted {
    //     total_sum += d.total_size(&directories);
    // }
    println!("The total sum of all under {} is: {}", max_size, total_sum)
}
