use std::fs;

#[derive(Debug)]
struct Session {
    root_dir: Dir,
    current_path: Vec<String>,
}

#[derive(Debug)]
struct Dir {
    name: String,
    entries: Vec<DirOrFile>,
}

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug)]
enum DirOrFile {
    Dir(Dir),
    File(File),
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

    println!("input lines {:?}", lines);

    let root_dir = Dir { name: "/".to_string(), entries: Vec::new() };
    // let dir_entry = Dir { name: "a_file".to_string(), entries: &mut Vec::new() };
    // let file_entry = File { name: "a_dir".to_string(), size: &12345 };
    // root_dir.entries.push(DirOrFile::Dir(dir_entry));
    // root_dir.entries.push(DirOrFile::File(file_entry));

    let mut session = Session { root_dir: root_dir, current_path: Vec::new() };

    session.parse_terminal_lines(lines);
    // println!("filesystem {:?}", root_dir);
}

impl Session {
    fn parse_terminal_lines(&mut self, lines: Vec<Vec<&str>>) {
        for words in lines.iter() {
            self.run_terminal_line(words);
        }
    }

    fn run_terminal_line(&mut self, words: &Vec<&str>) -> Option<String> {
        // get &mut Dir for self.cur_dir
        let mut curdir: &mut Dir = &mut self.root_dir;

        match words.iter().next() {
            Some(&"$") => {
                match words.get(1) {
                    Some(&"cd") => {
                        match words.get(1) {
                            Some(s) => return Some(s.to_string()),
                            _ => (),
                        }
                    },
                    Some(&"ls") => {},
                    _ => {},
                }
            },
            Some(&"dir") => {
                let d = Dir {
                    name: words.get(1).unwrap().to_string(),
                    entries: Vec::new(),
                };
                curdir.entries.push(DirOrFile::Dir(d));
            },
            Some(size) => {
                let f = File {
                    name: words.get(1).unwrap().to_string(),
                    size: size.parse::<usize>().unwrap(),
                };
                curdir.entries.push(DirOrFile::File(f));
            },
            _ => {},
        }
        println!("{:?}", words);
        None
    }
}

impl Dir {
    fn subdir(&self, name: &str) -> &Dir {
        for entry in self.entries.iter() {
            match entry {
                DirOrFile::Dir(d) => {
                    if d.name == name {
                        return d;
                    }
                },
                _ => (),
            }
        };
        self
    }
}
