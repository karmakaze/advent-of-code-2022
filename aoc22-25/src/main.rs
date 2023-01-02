use std::fs;
use std::str::FromStr;
use std::string::ParseError;

struct Snafu {
    value: i64,
    digits: Vec<i64>,
}

fn main() {
    let input_filename = "input25.txt";

    let content = fs::read_to_string(input_filename)
        .expect(format!("Unable to read the file \"{input_filename}\"").as_str());
    let input = content.as_str();
    let lines: Vec<&str> = input.lines().collect();

    let decimals: Vec<i64> = lines.iter().map(|s| Snafu::from_str(&s).unwrap().value).collect();
    for decimal in decimals.iter() {
        println!("{}", decimal);
    }

    let decimal_sum: i64 = decimals.iter().sum();
    println!("Part 1: decimal sum {} => snafu {}", decimal_sum, Snafu::from_value(decimal_sum).to_string());
}

impl Snafu {
    fn from_value(mut value: i64) -> Snafu {
        let mut snafu = Snafu { value: 0, digits: Vec::new() };
        if value == 0 {
            snafu.digits.push(0);
            return snafu;
        }

        let mut place: i64 = 1;
        while value != 0 {
            let m = match value % 5 {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 3, // -2
                4 => 4, // -1
                x => panic!("Snafu::from_value: unexpected {} % 5 -> {}", value, x),
            };
            if m <= 2 {
                snafu.value += m * place;
                snafu.digits.push(m);
                place *= 5;
                value = (value - m) / 5;
            } else {
                snafu.value += (m - 5) * place;
                snafu.digits.push(m - 5);
                place *= 5;
                value = (value - m + 5) / 5;
            }
        };
        snafu
    }

    fn digit_char(digit: &i64) -> String {
        match digit {
            -2 => "=",
            -1 => "-",
            0 => "0",
            1 => "1",
            2 => "2",
            _ => panic!("Snafu::digit_char: no char for digit {}", digit),
        }.to_string()
    }

    fn digit_value(ch: &char) -> i64 {
        match ch {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => panic!("Snafu::digit_value: no value for digit '{}'", ch),
        }
    }

    fn to_string(&self) -> String {
        if self.value == 0 {
            "0".to_string()
        } else {
            let chars: Vec<String> = self.digits.iter().map(Snafu::digit_char).rev().collect();
            chars.join("")
        }
    }
}

impl FromStr for Snafu {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {        
        let mut value = 0;
        let mut digits: Vec<i64> = Vec::new();
        for ch in s.chars() {
            let digit = match ch {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _ => panic!("crash and burn"),
            };
            digits.push(digit);
            value = value * 5 + digit;
        };

        Ok(Self {
            value: value,
            digits: digits,
        })
    }
}
