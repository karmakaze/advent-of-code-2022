use std::fs;
use std::str::FromStr;
use std::string::ParseError;
use regex::Regex;
use lazy_static::lazy_static;
// use std::io::{self, Write};

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn main() {
    let input_filename = "input5.txt";

    let contents = fs::read_to_string(input_filename)
        .expect(format!("Unable to read the file \"{input_filename}\"").as_str());
    let input_sections: Vec<&str> = contents.split("\n\n").collect();
    let input_stacks = input_sections[0];
    let input_moves = input_sections[1];

    // println!("input stacks:");
    // writeout(input_stacks).expect("Unable write stdout");
    // println!("\n");
    // println!("input moves:");
    // writeout(input_moves).expect("Unable write stdout");
    // println!();

    let mut stack: Vec<Vec<String>> = Vec::new();
    parse_stacks(input_stacks, &mut stack);

    // println!("input stack rows:");
    // for row in stack {
    //     println!("{:?}", row);
    // }

    transpose(&mut stack);
    print_stack("input stack cols:", &stack);

    let moves: Vec<Move> = input_moves.split("\n").filter_map(|s| {
        match s {
            "" => None,
            v => Some(v.parse::<Move>().unwrap()),
        }
    }).collect();
    println!("input moves:");
    for mv in moves.iter() {
        println!("{:?}", mv);
        apply_9000(&mut stack, mv);
        print_stack("stack:", &stack);
    }


    println!("Part 1: {:?}", stack_tops(&stack));

    stack.clear();
    parse_stacks(input_stacks, &mut stack);
    transpose(&mut stack);
    print_stack("starting stack cols:", &stack);
    for mv in moves.iter() {
        println!("{:?}", mv);
        apply_9001(&mut stack, mv);
        print_stack("stack:", &stack);
    }
    println!("Part 2: {:?}", stack_tops(&stack));
}

fn apply_9000(stack: &mut Vec<Vec<String>>, mv: &Move) {
    let mut count = mv.count;

    while count > 0 {
        let val = stack[mv.from].pop().unwrap();
        stack[mv.to].push(val);
        count -= 1;
    }
}

fn apply_9001(stack: &mut Vec<Vec<String>>, mv: &Move) {
    let mut count = mv.count;
    let mut tmp: Vec<String> = Vec::new();

    while count > 0 {
        let val = stack[mv.from].pop().unwrap();
        tmp.push(val);
        count -= 1;
    }
    while !tmp.is_empty() {
        let val = tmp.pop().unwrap();
        stack[mv.to].push(val);
    }
}

fn parse_stacks(input_stacks: &str, stack_rows: &mut Vec<Vec<String>>) {
    let stack_lines: Vec<&str> = input_stacks.split("\n").filter(|s| !s.is_empty()).collect();

    for line in stack_lines {
        let row: Vec<String> = line.chars().collect::<Vec<char>>()
            .chunks(4)
            .map(|chunk| chunk.iter().take(3).collect::<String>())
            .collect();
        stack_rows.push(row);
    };
}

fn transpose(rows: &mut Vec<Vec<String>>){
    let mut cols: Vec<Vec<String>> = Vec::new();
    while !rows.is_empty() {
        let mut row = rows.pop().unwrap();
        if cols.is_empty() {
            for _ in 0..(row.len()) {
                let col: Vec<String> = Vec::new();
                cols.push(col);
            }
        }
        let mut i: usize = 0;
        while !row.is_empty() {
            let val = row.pop().unwrap();
            if !val.trim().is_empty() {
                cols[i].push(val);
            }
            i += 1;
        }
    }

    while !cols.is_empty() {
        rows.push(cols.pop().unwrap());
    }
}

fn stack_tops(stack: &Vec<Vec<String>>) -> String {
    let tops: Vec<&str> = stack.iter().map(|col| {
        match col.last() {
            Some(v) => &(v.as_str()[1..2]),
            _ => " ",
        }
    }).collect();
    tops.join("")
}

fn print_stack(heading: &str, stack: &Vec<Vec<String>>) {
    println!("{heading}");
    for col in stack.iter() {
        println!("{:?}", col);
    }
}

impl FromStr for Move {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {        
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
        }

        let caps = RE.captures(line).unwrap();
        let count = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let from = caps.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let to = caps.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;

        Ok(Self {
            count: count,
            from: from,
            to: to,
        })
    }
}

// fn writeout(s: &str) -> io::Result<()> {
//     io::stdout().write_all(s.as_bytes())?;
//     Ok(())
// }
