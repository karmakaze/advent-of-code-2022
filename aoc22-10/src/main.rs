use std::fs;
use std::collections::HashSet;
use std::str::FromStr;
use std::string::ParseError;

#[derive(Debug)]
struct Machine {
    cycle: usize,
    instructions: Vec<Instruction>,
    pc_index: usize,
    x: i32,
}

#[derive(Debug)]
enum Instruction {
    NoOp(NoOp),
    AddX(AddX),
}

#[derive(Debug)]
struct NoOp {
}

#[derive(Debug)]
struct AddX {
    operand: i32,
}

fn main() {
    let input_filename = "input10.txt";

    let content = fs::read_to_string(input_filename)
        .expect(format!("Unable to read the file \"{input_filename}\"").as_str());
    let input = content.as_str();
    let instructions: Vec<Instruction> = input.lines().map(|s| Instruction::from_str(s).unwrap()).collect();

    // for instruction in instructions.iter() {
    //     println!("clocks {} {:?}", instruction.clocks(), instruction);
    // };

    let mut machine = Machine{
        cycle: 1,
        instructions: instructions,
        pc_index: 0,
        x: 1,
    };

    let mut sum_strengths: i64 = 0;
    let print_cycles: HashSet<usize> = HashSet::from([20, 60, 100, 140, 180, 220]);
    let mut cycle_log: Vec<String> = Vec::new();

    println!("{}", machine.summary());
    for i in 0..146 {
        let cycle_start = machine.cycle;
        let instruction_cycles = machine.instructions[machine.pc_index].cycles();
        let cycle_end = machine.cycle + instruction_cycles;
        let dot_clock = ((cycle_start - 1) % 40) as i32;

        let output_pixel = |dot_clock| {
            if machine.x - 1 <= dot_clock && dot_clock <= machine.x + 1 {
                print!("#");
            } else {
                print!(".");
            };
            if dot_clock == 39 {
                println!();
            };
        };
        output_pixel(dot_clock);
        if instruction_cycles > 1 {
            output_pixel(dot_clock + 1);
        }

        let cycle = if print_cycles.contains(&cycle_start) {
            Some(cycle_start)
        } else if print_cycles.contains(&(cycle_end - 1)) {
            Some(cycle_end - 1)
        } else { None };

        match cycle {
            Some(c) => {
                let strength = c as i64 * machine.x as i64;
                sum_strengths += strength;
                cycle_log.push(format!("{} => signal strength {}", machine.summary(), strength));
            },
            _ => (),
        };
        machine.step();
    }
    for line in cycle_log.iter() {
        println!("{}", line);
    }
    println!("Part 1: sum of signal strengths is {}", sum_strengths);
}

impl Machine {
    fn step(&mut self) {
        match &self.instructions[self.pc_index] {
            Instruction::NoOp(noop) => {
                self.cycle += noop.cycles();
            },
            Instruction::AddX(addx) => {
                self.x += addx.operand;
                self.cycle += addx.cycles();
            },
        }
        self.pc_index += 1;
        if self.pc_index >= self.instructions.len() {
            self.pc_index -= self.instructions.len();
        }
    }

    fn summary(&self) -> String {
        format!("{:>5}: {:<10} {:>3}:x {:>4}:cycle",
            self.pc_index, self.instructions[self.pc_index].to_string(), self.x, self.cycle)
    }
}

impl NoOp {
    fn cycles(&self) -> usize {
        1
    }
}

impl AddX {
    fn cycles(&self) -> usize {
        2
    }
}

impl Instruction {
    fn cycles(&self) -> usize {
        match self {
            Self::NoOp(noop) => noop.cycles(),
            Self::AddX(addx) => addx.cycles(),
        }
    }
}

impl ToString for Instruction {
    fn to_string(&self) -> String {
        match self {
            Self::NoOp(noop) => noop.to_string(),
            Self::AddX(addx) => addx.to_string(),
        }
    }
}

impl ToString for NoOp {
    fn to_string(&self) -> String {
        "noop".to_string()
    }
}

impl ToString for AddX {
    fn to_string(&self) -> String {
        format!("addx {}", self.operand)
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = line.split(" ").collect();
        let instruction: Instruction = match tokens.len() {
            1 => match NoOp::from_str(line) {
                Ok(noop) => Instruction::NoOp(noop),
                _ => panic!("Invalid instruction: {}", line),
            },
            2 => match AddX::from_str(line) {
                Ok(addx) => Instruction::AddX(addx),
                _ => panic!("Invalid instruction: {}", line),
            },
            _ => panic!("Invalid instruction: {:?}", tokens),
        };

        Ok(instruction)
    }
}

impl FromStr for NoOp {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = line.split(" ").collect();
        let noop: NoOp = match tokens.len() {
            1 => {
                match tokens.iter().next() {
                    Some(opcode) => {
                        match *opcode {
                            "noop" => Self{},
                            _ => panic!("Invalid instruction: {:?}", opcode),
                        }
                    },
                    _ => panic!("Invalid instruction: {}", line),
                }
            },
            _ => panic!("Invalid instruction: {:?}", tokens),
        };

        Ok(noop)
    }
}

impl FromStr for AddX {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = line.split(" ").collect();
        let addx = match tokens.len() {
            2 => {
                let mut iter = tokens.iter();
                match iter.next() {
                    Some(opcode) => {
                        match *opcode {
                            "addx" => Self{ operand: iter.next().unwrap().parse::<i32>().ok().unwrap() },
                            _ => panic!("Invalid instruction: {:?}", tokens),
                        }
                    },
                    _ => panic!("Invalid instruction: {:?}", tokens),
                }
            },
            _ => panic!("Invalid instruction: {:?}", tokens),
        };
        Ok(addx)
    }
}
