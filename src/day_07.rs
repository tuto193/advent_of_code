use crate::get_file_contents;

struct Feil {
    size: usize,
}

struct Deirectory {
    name: &str,
    parent: Option<&str>,
    dirs: Vec<Deirectory>,
    feils: Vec<Feil>,
}

impl Deirectory {
    pub fn total_size(&self) -> usize {
        let sum = 0;
        for f in self.feils.into_iter() {
            sum += f.size;
        }
        for d in self.dirs.into_iter() {
            sum += d.total_size();
        }
        sum
    }

    pub fn set_parent(&mut self, p_name: &str) {
        match self.parent {
            Some(n) => println!("Tried to set parent from {} to {}", n, p_name),
            None => self.parent = Some(p_name),
        }
    }

    pub fn add_feil(&mut self, feil: Feil) {
        self.feils.push(feil)
    }

    pub fn add_dir(&mut self, dir: Deirectory) {
        self.dirs.push(dir)
    }

    pub fn show_parent(&self) -> &str {
        match self.parent {
            Some(p) => p,
            None => {
                println!("I has no parents");
                ""
            },
        }
    }

    pub fn show_feils(&self) -> &Vec<Feil> {
        &self.feils
    }

    pub fn show_dirs(&self) -> &Vec<Deirectory> {
        &self.dirs
    }
}

pub fn day_07() {
    let input = get_file_contents("07".to_string());
}
