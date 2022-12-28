use crate::get_file_contents;

struct File {
    size: usize,
    name: String,
}

struct Directory {
    name: String,
    parent: Option<Box<Directory>>,
    dirs: Vec<Directory>,
    files: Vec<File>,
}

impl Directory {
    pub fn new(name: String, parent: Option<Directory>) -> Self {
        Self{
            name: name,
            parent: parent,
            dirs: vec![],
            files: vec![],
        }
    }

    pub fn total_size(&self) -> usize {
        let sum = 0;
        for f in self.files.into_iter() {
            sum += f.size;
        }
        for d in self.dirs.into_iter() {
            sum += d.total_size();
        }
        sum
    }

    pub fn set_parent(&mut self, p: Directory) {
        match self.parent {
            Some(n) => println!("Tried to set parent from {} to {}", n.name, p.name),
            None => self.parent = Some(Box::new(p)),
        }
    }

    pub fn add_file(&mut self, file: File) {
        self.files.push(file)
    }

    pub fn add_dir(&mut self, dir: Directory) {
        self.dirs.push(dir)
    }

    pub fn show_parent(&self) -> Option<Directory> {
        match self.parent {
            Some(p) => Some(*p),
            None => {
                println!("I has no parents");
                None
            },
        }
    }

    pub fn show_files(&self) -> &Vec<File> {
        &self.files
    }

    pub fn show_dirs(&self) -> &Vec<Directory> {
        &self.dirs
    }
}

impl File {
    pub fn new(size: usize) -> Self {
        Self{
            size: size,
        }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }
}

pub fn day_07() {
    let input = get_file_contents("07".to_string());
}
