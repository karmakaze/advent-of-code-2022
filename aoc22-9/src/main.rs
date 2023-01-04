use std::fs;
use std::collections::HashSet;
use std::str::FromStr;
use std::string::ParseError;
use std::convert::Infallible;

#[derive(Debug)]
struct Board {
    width: i32,
    height: i32,
    
    head: Point,
    knots: Vec<Point>,
}

#[derive(Debug)]
#[derive(Eq, Hash, PartialEq)]
#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Command {
    dir: Dir,
    count: usize,
}

#[derive(Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let input_filename = "input9.txt";

    let content = fs::read_to_string(input_filename)
        .expect(format!("Unable to read the file \"{input_filename}\"").as_str());
    let input = content.as_str();

    let commands: Vec<Command> = input.lines().filter_map(|line| Command::from_str(line).ok()).collect();

    let mut head = Point { x: 11, y: 5 };
    let mut knots: Vec<Point> = Vec::new();
    for _ in 1..=9 { knots.push(head.clone()) };
    let mut board = Board { head: head, knots: knots, width: 12, height: 6 };
    println!("Initial");
    board.print();

    let mut tail_visits: HashSet<Point> = HashSet::new();
    tail_visits.insert(board.tail());

    for command in commands.iter() {
        println!();
        println!("cmd: {:?}", command);
        for _ in 0..command.count {
            board.move_head(&command.dir);
            board.move_knots();
            tail_visits.insert(board.tail());
        }
        board.print();
    };

    println!("Part 2: Tail visited {} positions.", tail_visits.len());
    tail_visits.clear();
}

impl Board {
    fn move_head(&mut self, dir: &Dir) {
        let next_head = match dir {
            Dir::Up => self.head.up(),
            Dir::Down => self.head.down(),
            Dir::Left => self.head.left(),
            Dir::Right => self.head.right(),
        };
        if next_head.x >= self.width {
            self.width = next_head.x + 1;
        }
        if next_head.y >= self.height {
            self.height = next_head.y + 1;
        }
        // println!("head {:?} {:?} => {:?}", self.head, dir, next_head);
        self.head = next_head;
    }

    fn move_knots(&mut self) {
        let mut prev_knot = self.head.clone();
        for i in 0..self.knots.len() {
            let mut knot: &mut Point = self.knots.get_mut(i).unwrap();
            if !knot.touching(&prev_knot) {
                let dir = Point{ x: cmp(prev_knot.x, knot.x), y: cmp(prev_knot.y, knot.y) };
                *knot = knot.move_dir(dir);
            };
            prev_knot = knot.clone();
        };
    }

    fn tail(&self) -> Point {
        self.knots.last().unwrap().clone()
    }

    fn print(&self) {
        for row in (0..self.height).rev() {
            for col in 0..self.width {
                let cell = Point { x: col, y: row };

                if self.head == cell {
                    print!("H")
                } else {
                    match self.knots.iter().enumerate().find(|(_, knot)| **knot == cell) {
                        Some((i, _)) => print!("{}", i + 1),
                        _ => print!("."),
                    }
                }
            }
            println!();
        }
    }
}

impl Point {
    fn move_dir(&self, dir: Self) -> Self {
        Self{ x: self.x + dir.x, y: self.y + dir.y }
    }

    fn up(&self) -> Self {
        Self{ x: self.x, y: self.y + 1 }
    }

    fn down(&self) -> Self {
        Self{ x: self.x, y: self.y - 1 }
    }

    fn left(&self) -> Self {
        Self{ x: self.x - 1, y: self.y }
    }

    fn right(&self) -> Self {
        Self{ x: self.x + 1, y: self.y }
    }

    fn touching(&self, other: &Self) -> bool {
        (self.y - other.y).abs() <= 1 && (self.x - other.x).abs() <= 1
    }
}

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Infallible> {
        let mut tokens = line.split(" ");
        let dir = match tokens.next().unwrap() {
            "U" => Dir::Up,
            "D" => Dir::Down,
            "L" => Dir::Left,
            "R" => Dir::Right,
            d => panic!("Invalid dir '{}'", d),
        };

        let count = match tokens.next().unwrap().parse::<usize>() {
            Ok(c) => c,
            Err(e) => panic!("Invalid move count {}", e),
        };

        Ok(Self{ dir: dir, count: count })
    }
}

fn cmp(a: i32, b: i32) -> i32 {
    if a < b {
        -1
    } else if a > b {
        1
    } else {
        0
    }
}
