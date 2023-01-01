use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Session {
    root_dir: Rc<RefCell<Dir>>,
    curr_dirs: Vec<Rc<RefCell<Dir>>>,
    curr_path: Vec<String>,
    last_ls_dir: Option<Rc<RefCell<Dir>>>,
    last_ls_seen: HashSet<String>,
}

#[derive(Debug)]
struct Dir {
    name: String,
    entries: HashMap<String, DirOrFile>,
}

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug)]
enum DirOrFile {
    Dir(Rc<RefCell<Dir>>),
    File(Rc<RefCell<File>>),
}

fn main() {
    let input_filename = "input7.txt";

    let content = fs::read_to_string(input_filename)
        .expect(format!("Unable to read the file \"{input_filename}\"").as_str());
    let input = content.as_str();
    let lines: Vec<Vec<&str>> = input.split("\n").filter_map(|line| {
        if line.is_empty() {
            None
        } else {
            Some(line.split(" ").collect())
        }
    }).collect();

    let root_dir = Dir { name: "/".to_string(), entries: HashMap::new() };
    let mut session = Session {
        root_dir: Rc::new(RefCell::new(root_dir)),
        curr_dirs: Vec::new(),
        curr_path: Vec::new(),
        last_ls_dir: None,
        last_ls_seen: HashSet::new(),
    };
    session.add_dir("/".to_string());

    session.parse_terminal_lines(lines);
    session.flush_last_ls();

    let mut dir_sizes: HashMap<String, usize> = HashMap::new();

    session.root_dir.borrow().sizes_recursive(&"/".to_string(), &mut dir_sizes);

    // println!("Dir sizes:");
    // for dir_size in dir_sizes.values() {
    //     println!("{}", dir_size);
    // }

    let small_dir_sizes: Vec<usize> = dir_sizes.values().filter_map(|&size| {
        if size <= 100000 {
            Some(size)
        } else {
            None
        }
    }).collect();

    // println!("Small dir sizes:");
    // for dir_size in small_dir_sizes.iter() {
    //     println!("{}", dir_size);
    // }
    println!("Part 1: sum of small dir sizes {}", small_dir_sizes.iter().sum::<usize>());

    let total_disk_space = 70000000;
    let free_space_required = 30000000;
    let total_used_space = dir_sizes.get(&"//".to_string()).unwrap();
    let total_free_space = total_disk_space - total_used_space;
    let free_space_needed = free_space_required - total_free_space;

    println!("  {} total_disk_space", total_disk_space);
    println!("- {} total_used_space", total_used_space);
    println!("---------------------------");
    println!("  {} total_free_space", total_free_space);
    println!();
    println!("  {} free_space_required", free_space_required);
    println!("- {} total_free_space", total_free_space);
    println!("---------------------------");
    println!("  {} free_space_needed", free_space_needed);


    let mut right_path: String = "".to_string();
    let mut right_size: usize = total_disk_space + 1;
    for (path, size) in dir_sizes.iter() {
        if *size >= free_space_needed && *size < right_size {
            right_path = path.clone();
            right_size = *size;
        }
    }
    println!("Part 2: delete directory '{right_path}' size {right_size} to make {} free space", total_free_space + right_size);
}

impl Session {
    fn parse_terminal_lines(&mut self, lines: Vec<Vec<&str>>) {
        for words in lines.iter() {
            // println!("{}", words.join(" "));
            self.parse_terminal_line(&words);
            // println!("< {:?}", self.curr_path);
        }
    }

    fn parse_terminal_line(&mut self, words: &Vec<&str>) {
        match words.iter().next() {
            Some(&"$") => {
                self.flush_last_ls();

                match words.get(1) {
                    Some(&"cd") => {
                        match words.get(2) {
                            Some(cd_name) => {
                                self.chdir(cd_name);
                            },
                            _ => (),
                        }
                    },
                    Some(&"ls") => {
                        let curr_dir = self.curr_dirs.last().unwrap();
                        self.last_ls_dir = Some(curr_dir.clone());
                    },
                    _ => (),
                }
            },
            Some(&"dir") => {
                let name = words.get(1).unwrap().to_string();
                self.last_ls_seen.insert(name.clone());
                let curr_dir = self.curr_dirs.last().unwrap();
                let existing = curr_dir.borrow().get_dir(name.clone());
                if existing.is_none() {
                    // no method `add_dir` found for reference `&Rc<RefCell<Dir>>` in the current scope
                    curr_dir.borrow_mut().add_dir(name);
                }
            },
            Some(size) => {
                let name = words.get(1).unwrap().to_string();
                self.last_ls_seen.insert(name.clone());
                self.add_file(name, size);
            },
            _ => {},
        }
    }

    fn add_dir(&mut self, name: String) {
        if self.curr_dirs.is_empty() && name.eq("/") {
            self.curr_dirs.push(self.root_dir.clone());
            self.curr_path.push(name);
        } else {
            println!("< Adding dir '{}' to {}", name.clone(), self.curr_path.join("/"));
            let d = Dir {
                name: name.clone(),
                entries: HashMap::new(),
            };
            let dir = Rc::new(RefCell::new(d));
            let curr_dir = self.curr_dirs.last().unwrap();
            curr_dir.borrow_mut().entries.insert(name, DirOrFile::Dir(dir));
        }
    }

    fn add_file(&mut self, name: String, size: &str) {
        let curr_dir = self.curr_dirs.last().unwrap();
        curr_dir.borrow_mut().add_file(name, size.parse::<usize>().unwrap());
    }

    fn chdir(&mut self, name: &str) {
        if name.eq("..") {
            // println!("DBG: cd ..");
            self.curr_dirs.pop();
            self.curr_path.pop();
            return;
        } else if name.eq("/") {
            // println!("DBG: cd /");
            while self.curr_dirs.len() > 1 {
                self.curr_dirs.pop();
                self.curr_path.pop();
            }
            return;
        }
        // println!("DBG: cd {}/{}", self.curr_path.join("/"), name);
        let curr_dir = self.curr_dirs.last().unwrap();
        let ch_dir = curr_dir.borrow().get_dir(name.to_string());
        if ch_dir.is_some() {
            let ch_d = ch_dir.as_ref().unwrap();
            self.curr_dirs.push(ch_d.clone());
            self.curr_path.push(name.to_string());
        }
    }

    fn flush_last_ls(&mut self) {
        if self.last_ls_dir.is_some() {
            let mut ls_dir = self.last_ls_dir.as_mut().unwrap().borrow_mut();
            let mut unseen: HashSet<String> = HashSet::new();
            for key in ls_dir.entries.keys() {
                if !self.last_ls_seen.contains(key) {
                    // println!("Unseen {}", key);
                    unseen.insert(key.clone());
                // } else {
                //     println!("Seen {}", key);
                }
            }
            for key in unseen {
                // println!("Removing unseen {}", &key);
                ls_dir.entries.remove(&key);
            }
        }
        self.last_ls_dir = None;
        self.last_ls_seen.clear();
    }
}

impl Dir {
    fn sizes_recursive(&self, parent_path: &String, mut fullname_sizes: &mut HashMap<String, usize>) -> usize {
        let path = format!("{parent_path}{}", self.name);
        match fullname_sizes.get(&path) {
            Some(size) => { return *size },
            None => (),
        }

        let mut size: usize = 0;
        for entry in self.entries.values() {
            match entry {
                DirOrFile::File(f) => {
                    size += f.borrow().size;
                },
                DirOrFile::Dir(d) => {
                    size += d.borrow().sizes_recursive(&path, &mut fullname_sizes);
                },
            }
        };
        fullname_sizes.insert(path.clone(), size);
        size
    }

    fn get_dir(&self, name: String) -> Option<Rc<RefCell<Dir>>> {
        match self.entries.get(&name) {
            Some(DirOrFile::Dir(d)) => Some(d.clone()),
            _ => None
        }
    }

    fn get_file(&self, name: &String) -> Option<Rc<RefCell<File>>> {
        match self.entries.get(name) {
            Some(DirOrFile::File(f)) => Some(f.clone()),
            _ => None
        }
    }

    fn add_dir(&mut self, name: String) {
        if self.get_dir(name.clone()).is_none() {
            let d = Dir {
                name: name.clone(),
                entries: HashMap::new(),
            };
            self.entries.insert(name, DirOrFile::Dir(Rc::new(RefCell::new(d))));
        }
    }

    fn add_file(&mut self, name: String, size: usize) {
        let found = self.get_file(&name);
        match found {
            Some(f) => {
                f.borrow_mut().size = size;
            },
            _ => {
                let f = File {
                    name: name.clone(),
                    size: size,
                };        
                self.entries.insert(name, DirOrFile::File(Rc::new(RefCell::new(f))));
            },
        }
    }

    fn ls_lr(&self, indent: String, parent: String) {
        for entry in self.entries.values() {
            match entry {
                DirOrFile::Dir(d) => {
                    let dir = d.borrow();
                    println!("{indent}dir {}", dir.name);
                    dir.ls_lr(format!("{indent}  "), format!("{parent}/{}", self.name));
                },
                DirOrFile::File(f) => {
                    let file = f.borrow();
                    println!("{indent}{} {}", file.size, file.name);
                },
            }    
        }
    }
}
