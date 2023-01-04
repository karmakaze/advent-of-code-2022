use std::fs;
use std::cmp::Ordering;
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug)]
#[derive(Clone)]
enum ValueList {
    Value(i32),
    List(Vec<ValueList>),
}

fn main() {
    let input_filename = "input13.txt";

    let content = fs::read_to_string(input_filename)
        .expect(format!("Unable to read the file \"{input_filename}\"").as_str());
    let input = content.as_str();

    let pairs: Vec<(ValueList, ValueList)> = input.split("\n\n").map(|pair| {
        let mut lines = pair.lines();
        let mut first = tokenize(lines.next().unwrap());
        let mut second = tokenize(lines.next().unwrap());
        // println!("tokens {:?}, tokens {:?}", first.join(""), second.join(""));
        (ValueList::parse(&mut first).unwrap(), ValueList::parse(&mut second).unwrap())
    }).collect();

    let mut indices: Vec<usize> = Vec::new();
    for (i, pair) in pairs.iter().enumerate() {
        let c = pair.0.cmp(&pair.1);
        // println!("#{} <{}> ({:?}, {:?})", i + 1, c, pair.0, pair.1);
        if c == Ordering::Less {
            indices.push(i + 1);
        }
    }
    let sum: usize = indices.iter().sum();
    println!("Part 1: sum of ordered pair indices is {}", sum);

    let mut all_packets: Vec<ValueList> = pairs.into_iter().flat_map(|pair| [pair.0, pair.1]).collect();
    let marker2 = ValueList::parse(&mut tokenize("[[2]]")).unwrap();
    let marker6 = ValueList::parse(&mut tokenize("[[6]]")).unwrap();
    all_packets.push(marker2.clone());
    all_packets.push(marker6.clone());

    println!();
    println!("Ordered packets:");
    all_packets.sort();
    for packet in all_packets.iter() {
        println!("{:?}", packet);
    }
    let pos2 = all_packets.iter().enumerate().find(|(_, packet)| **packet == *&marker2 );
    let pos6 = all_packets.iter().enumerate().find(|(_, packet)| **packet == *&marker6 );
    println!("Part 2: product of marker indices {}", (pos2.unwrap().0 + 1) * (pos6.unwrap().0 + 1));
}

fn tokenize(line: &str) -> Vec<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[\[,\]]").unwrap();
    }

    let mut tokens: Vec<&str> = Vec::new();
    let mut index: usize = 0;
    for found in RE.find_iter(line) {
        if index < found.start() {
            tokens.push(&line[index..found.start()]);
        };
        tokens.push(&line[found.start()..found.end()]);
        index = found.end();
    }
    if index < line.len() {
        tokens.push(&line[index..line.len()]);
    }
    tokens.reverse();
    tokens
}

impl PartialEq for ValueList {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for ValueList {}

impl PartialOrd for ValueList {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for ValueList {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl ValueList {    
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Self::Value(v) => {
                match other {
                    Self::Value(w) => {
                        cmp(*v, *w)
                    },
                    Self::List(_) => {
                        Self::List(vec![Self::Value(*v)]).cmp(other)
                    },
                }
            },
            Self::List(l) => {
                match other {
                    Self::Value(w) => {
                        self.cmp(&Self::List(vec![Self::Value(*w)]))
                    },
                    Self::List(m) => {
                        let mut iter1 = l.iter();
                        let mut iter2 = m.iter();
                        loop {
                            match iter1.next() {
                                Some(a) => {
                                    match iter2.next() {
                                        Some(b) => {
                                            let c = a.cmp(b);
                                            if c != Ordering::Equal {
                                                return c;
                                            }
                                        },
                                        None => return Ordering::Greater,
                                    }        
                                },
                                None => {
                                    match iter2.next() {
                                        Some(_) => return Ordering::Less,
                                        None => return Ordering::Equal,
                                    }        
                                },
                            };
                        }
                    },
                }
            },
        }
    }

    fn parse(mut tokens: &mut Vec<&str>) -> Option<Self> {
        let first = tokens.pop();
        match first {
            None => None,
            Some(token) => {
                if token == "[" {
                    let mut values: Vec<ValueList> = Vec::new();
                    loop {
                        match Self::parse(&mut tokens) {
                            Some(vl) => values.push(vl),
                            None => (),
                        }
                        match tokens.last() {
                            Some(delim) => {
                                if *delim == "," {
                                    tokens.pop();
                                } else if *delim == "]" {
                                    tokens.pop();
                                    break;
                                }
                            },
                            None => break,
                        }
                    }
                    Some(Self::List(values))
                } else if token == "]" {
                    tokens.push(token);
                    None
                } else {
                    Some(Self::Value(token.parse::<i32>().ok().unwrap()))
                }
            },
        }
    }
}

fn cmp(a: i32, b: i32) -> Ordering {
    if a < b {
        Ordering::Less
    } else if a > b {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}
