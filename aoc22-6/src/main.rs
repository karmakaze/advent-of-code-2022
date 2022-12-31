use std::fs;

#[derive(Debug)]
struct StartDetector<'a> {
    count: usize,
    size: Option<usize>,
    read: usize,
    buffer: &'a str,
}

fn main() {
    let input_filename = "input6.txt";

    let content = fs::read_to_string(input_filename)
        .expect(format!("Unable to read the file \"{input_filename}\"").as_str());
    let input = content.as_str();
    // let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb"; // first marker after character 19
    // let input = "bvwbjplbgvbhsrlpgdmjqwftvncz"; // first marker after character 23
    // let input = "nppdvjthqldpwncqszvftbrmjlhg"; // first marker after character 23
    // let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"; //  first marker after character 29
    // let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"; //  first marker after character 26

    let mut detector = StartDetector {
        count: 4,
        size: Option::None,
        read: 0,
        buffer: "",
    };
    while detector.is_looking() {
        detector.step(input);
    };
    println!("Part 1: start detected read {:?} saw {:?}", detector.read, detector.buffer);

    detector = StartDetector {
        count: 14,
        size: Option::None,
        read: 0,
        buffer: "",
    };
    while detector.is_looking() {
        detector.step(input);
    };
    println!("Part 2: start detected read {:?} saw {:?}", detector.read, detector.buffer);
}

impl<'a> StartDetector<'a> {
    fn is_looking(&self) -> bool {
        self.size.is_none() || self.read < self.size.unwrap() && !self.is_found()
    }

    fn is_found(&self) -> bool {
        if self.buffer.len() < self.count {
            return false;
        }
        let mut seen: Vec<char> = Vec::new();
        for c in self.buffer.chars() {
            if seen.contains(&c) {
                return false;
            }
            seen.push(c);
        }
        true
    }

    fn step(&mut self, input: &'a str) {
        if self.size.is_none() {
            self.size = Some(input.len());
        }
        self.read += 1;
        self.buffer = if self.read < self.count {
            &input[0..self.read]
        } else {
            &input[(self.read - self.count)..self.read]
        }
    }
}
