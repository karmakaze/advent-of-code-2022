use std::fs;

#[derive(Debug)]
struct Crypt {
    indices: Vec<usize>,
    digits: Vec<i64>,
}

fn main() {
    let input_filename = "input20-test.txt";

    let content = fs::read_to_string(input_filename)
        .expect(format!("Unable to read the file \"{input_filename}\"").as_str());
    let input = content.as_str();
    let lines: Vec<&str> = input.lines().collect();

    let mut digits: Vec<i64> = lines.iter().map(|s| s.parse::<i64>().unwrap()).collect();
    for digit in digits.iter() {
        println!("{}", digit);
    }

    let indices = (0..digits.len()).rev().collect();
    let mut crypt = Crypt { digits: digits, indices: indices };
    crypt.run();

    match crypt.digits.iter().position(|digit| *digit == 0) {
        Some(i) => {
            // Then, the grove coordinates can be found by looking at the 1000th, 2000th, and 3000th numbers
            // after the value 0, wrapping around the list as necessary. In the above example,
            // the 1000th number after 0 is 4, the 2000th is -3, and the 3000th is 2;
            // adding these together produces 3.
            println!("Position {} found '0'.", i);
            let digits_len = crypt.digits.len();
            let char1000 = crypt.digits[(i + 1000) % digits_len];
            let char2000 = crypt.digits[(i + 2000) % digits_len];
            let char3000 = crypt.digits[(i + 3000) % digits_len];
            println!("Chars at positions 1000, 2000, 3000 after '0' are {} {} {}", char1000, char2000, char3000);
        },
        _ => {
            println!("No '0' in input.");
        },
    }

}

impl Crypt {
    fn run(&mut self) {
        println!("initial: {:?}", self);

        while !self.indices.is_empty() {
            self.apply();
            println!("after: {:?}", self);
        };
    }

    fn apply(&mut self) {
        let from_index = self.indices.pop().unwrap();
        let digit = self.digits[from_index];
        let digits_len = self.digits.len() as i64;
        // println!("DBG: moving {} at {}", digit, from_index);
        if digit == 0 || digit.abs() % (digits_len - 1) == 0 {
            println!("no-op");
            return;
        };

        let mut to_i = (from_index as i64 + digit) % (digits_len - 1);
        if to_i < 0 {
            to_i += digits_len - 1;
        };

        if to_i == 0 && from_index > 0 {
            to_i = digits_len - 1;
        }
        // println!("shuffle {digit} from {from_index} dest {to_i}");
        self.shuffle(digit, from_index, to_i as usize);    
}

    fn shuffle(&mut self, digit: i64, from: usize, dest: usize) {
        match cmp(dest, from) {
            1 => {
                // println!("a) Removing {} from position {}", self.digits.remove(from), from);
                self.digits.remove(from);

                // println!("Inserting {} at position {}", digit, dest);
                self.digits.insert(dest, digit);
    
                let mut slice: &mut [usize] = self.indices.as_mut_slice();
                println!("Decrementing indices {}..={}", from + 1, dest);
                for i in 0..slice.len() {
                    let index: usize = slice[i as usize];
                    if from < index && index <= dest {
                        slice[i as usize] -= 1;
                    }
                };
            },
            -1 => {
                // println!("b) Removing {} from position {}", self.digits.remove(from), from);
                self.digits.remove(from);

                // println!("Inserting {} at position {}", digit, dest);
                self.digits.insert(dest, digit);
    
                let mut slice: &mut [usize] = self.indices.as_mut_slice();
                // println!("Decrementing indices {}..={}", from + 1, dest);
                for i in 0..slice.len() {
                    let index: usize = slice[i as usize];
                    if from < index && index <= dest {
                        slice[i as usize] -= 1;
                    }
                };    
            },
            _ => {
                println!("DBG: Hey! from {} dest {}", from, dest);
            },
        }
    }
}

fn cmp(x: usize, y: usize) -> i32 {
    if x < y {
        -1
    } else {
        if x > y {
            1
        } else {
            0
        }
    }
}
