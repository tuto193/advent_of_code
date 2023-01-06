use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Clone)]
pub struct File {
    name: String,
    size: usize,
}
#[derive(Clone, Debug)]
pub struct Directory {
    name: String,
    parent: Option<Rc<RefCell<Directory>>>,
    // Test
    dirs: HashMap<String, Rc<RefCell<Directory>>>,
    files: Vec<File>,
}

impl Directory {
    pub fn new(name: String, parent: Option<Rc<RefCell<Directory>>>) -> Self {
        Self {
            name: name,
            parent: parent,
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
        sum
    }

    pub fn add_file(&mut self, size: usize, name: String) {
        let to_add = File {
            name: name,
            size: size,
        };
        self.files.push(to_add)
    }

    pub fn add_dir(&mut self, dir: Rc<RefCell<Directory>>) {
        let name = &dir.borrow().name;
        self.dirs.insert(name.clone(), Rc::clone(&dir));
    }

    pub fn get_parent(&self) -> Option<Rc<RefCell<Directory>>> {
        // never use `..` on `/`
        match &self.parent {
            None => {
                println!("Tried to get parent of {}, but it's orphan.", self.name);
                None
            }
            Some(p) => Some(Rc::clone(&p)),
        }
        // to_show
    }

    /// Returns a vector of all files in this and all its subdirectories
    pub fn get_files(&self) -> Vec<File> {
        let mut to_return = self.files.clone();
        for (_, dir) in self.dirs.iter() {
            // println!("Dir '{}' is in {}", n, self.name);
            let dir = Rc::clone(dir);
            let mut subdir_files = dir
                .borrow_mut()
                .get_files()
                .into_iter()
                .collect::<Vec<File>>();
            to_return.append(&mut subdir_files);
        }
        to_return
    }

    pub fn get_dirs(&mut self) -> &mut HashMap<String, Rc<RefCell<Directory>>> {
        &mut self.dirs
    }

    pub fn get_all_under_limit(&self, max_limit: usize) -> usize {
        let mut total_sum = 0;
        let this_dirs_size = self.total_size();
        if this_dirs_size <= max_limit {
            total_sum += this_dirs_size;
        }
        for (_n, d) in self.dirs.iter() {
            let d = Rc::clone(d);
            total_sum += d.borrow_mut().get_all_under_limit(max_limit);
        }
        total_sum
    }

    pub fn print_hierarchy(&self, depth: usize) {
        println!("|");
        print!("|");
        for _ in 0..(depth - 1) * 4 {
            print!("-")
        }
        println!("|>{}", self.name);
        for f in self.files.iter() {
            print!("|");
            for _ in 0..depth * 4 {
                print!("-")
            }
            println!("++{} [{}]", f.name, f.size);
        }
        for (_n, d) in self.dirs.iter() {
            let d = Rc::clone(d);
            d.borrow_mut().print_hierarchy(depth + 1);
        }
    }

    pub fn get_all_over_size(&self, max_size: usize) -> Vec<usize> {
        let mut to_return: Vec<usize> = vec![];
        if self.total_size() >= max_size {
            // println!("Dir '{}' has size '{}' which is under '{}'", self.name, self.total_size(), max_size);
            to_return.push(self.total_size());
            for (_n, d) in self.dirs.iter() {
                let mut d = d.borrow().get_all_over_size(max_size);
                to_return.append(&mut d);
            }
        }
        to_return
    }
}

pub fn get_directories_from_input(input: String) -> Rc<RefCell<Directory>> {
    // let mut dirs = vec![];
    let commands = input.split("$ ").collect::<Vec<&str>>();
    // let mut active_dirs: Vec<String> = vec![];
    let mut active_directory = Rc::new(RefCell::new(Directory::new("/".to_string(), None)));
    for command in commands.into_iter().skip(1) {
        // println!("Command looks like ' {} '.", command);
        let lines = command.split("\n").collect::<Vec<&str>>();
        // println!("Lines after split are now: '{:?}'", lines);
        if lines.len() == 2 {
            /////////////////// It is a `cd` command = new directory
            let cd_command_line = lines[0].split(" ").collect::<Vec<&str>>();
            // println!("Command line was length one and looks like: {:?}", cd_command_line);
            match cd_command_line[1] {
                ".." => {
                    // go back to parent dir
                    // If this ever panics, it's because we tried to `..` at `/`
                    // TEST: Rcs and RefCells
                    let parent_dir = active_directory.borrow_mut().get_parent().unwrap();
                    // active_directory.
                    active_directory = parent_dir;
                }
                "/" => continue,
                dir_name => {
                    // println!("Directory name is '{}'", dir_name);
                    // Keep track of our current directory's name
                    // active_dirs.push(dir_name.to_string());
                    ///// TEST # 3: Rcs and Refcells
                    let next_subdir = Rc::clone(&active_directory);
                    let mut next_subdir = next_subdir.borrow_mut();
                    let next_subdir = next_subdir.get_dirs().get(dir_name).unwrap();
                    active_directory = Rc::clone(&next_subdir);
                }
            }
        } else {
            //////////////// it's an `ls` command = we need to split check
            // Skip the `ls` part
            for line in lines.into_iter().skip(1) {
                let output = line.split(" ").collect::<Vec<&str>>();
                // let active_dir = dirs.last_mut().unwrap();
                match output[0] {
                    "dir" => {
                        let found_dir_name = output[1];
                        // println!("Dir '{}': added dir '{}'", active_dir.name, output[1]);
                        // Test: Rcs and RefCells
                        let found_child_dir = Rc::new(RefCell::new(Directory::new(
                            found_dir_name.to_string(),
                            Some(Rc::clone(&active_directory)),
                        )));
                        active_directory.borrow_mut().add_dir(found_child_dir);
                    }
                    "" => continue,
                    f_size => {
                        let file_size: usize = f_size.parse().unwrap();
                        active_directory
                            .borrow_mut()
                            .add_file(file_size, output[1].to_string());
                        // println!("Dir '{}': added file '{}'", active_dir.name, output[1]);
                    }
                }
            }
        }
    }
    // dirs
    loop {
        {
            let my_clone = Rc::clone(&active_directory);
            match my_clone.borrow_mut().get_parent() {
                None => break,
                Some(_p) => {
                    // let parent = Rc::clone(&active_directory);
                }
            };
        }
        // active_directory = active_directory.borrow_mut().get_parent().unwrap();
        let parent = active_directory.borrow_mut().get_parent().unwrap();
        active_directory = Rc::clone(&parent);
    }
    active_directory
}
