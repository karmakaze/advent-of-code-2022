use std::fs;

#[derive(Debug)]
struct Dir<'a> {
    name: String,
    entries: &'a mut Vec<DirOrFile<'a>>,
}

#[derive(Debug)]
struct File<'a> {
    name: String,
    size: &'a usize,
}

#[derive(Debug)]
enum DirOrFile<'a> {
    Dir(Dir<'a>),
    File(File<'a>),
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

    let mut root_dir = Dir { name: "/".to_string(), entries: &mut Vec::new() };
    // let dir_entry = Dir { name: "a_file".to_string(), entries: &mut Vec::new() };
    // let file_entry = File { name: "a_dir".to_string(), size: &12345 };
    // root_dir.entries.push(DirOrFile::Dir(dir_entry));
    // root_dir.entries.push(DirOrFile::File(file_entry));

    parse_terminal_lines(lines, &mut root_dir);
    // println!("filesystem {:?}", root_dir);
}

fn parse_terminal_lines<'a>(lines: Vec<Vec<&str>>, root_dir: &'a mut Dir<'a>) {
    let mut cur_path: Vec<&'a Dir> = Vec::new();
    cur_path.push(root_dir);

    lines.iter().for_each(|words| {
        match words.iter().next() {
            Some(&"$") => {
                match words.get(1) {
                    Some(&"cd") => {
                        match words.get(1) {
                            Some(&"..") => {
                                cur_path.pop();
                            }
                            Some(name) => {
                                cur_path.push(cur_path.last().unwrap().subdir(name))
                            }
                            _ => ()
                        }
                    },
                    Some(&"ls") => {},
                    _ => {},
                }
            },
            Some(&"dir") => {
                let d = Dir {
                    name: words.get(1).unwrap().to_string(),
                    entries: &mut Vec::new(),
                };
                cur_path.last().unwrap().entries.push(DirOrFile::Dir(d));
            },
            Some(size) => {
                let f = File {
                    name: words.get(1).unwrap().to_string(),
                    size: &size.parse::<usize>().unwrap(),
                };
                cur_path.last().unwrap().entries.push(DirOrFile::File(f));
            },
            _ => {},
        }
        println!("{:?}", words);
    });
}

impl<'a> Dir<'a> {
    fn subdir(&'a self, name: &str) -> &'a Dir<'a> {
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