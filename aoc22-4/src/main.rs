use std::fs;
use std::ops::RangeInclusive;

fn main() {
    let input_filename = "input4.txt";

    let contents = fs::read_to_string(input_filename)
        .expect(format!("Unable to read the file \"{input_filename}\"").as_str());
    let input_lines: Vec<&str> = contents.split("\n").filter(|s| !s.is_empty()).collect();

    let input_range_pairs: Vec<(RangeInclusive<i32>, RangeInclusive<i32>)> = input_lines.iter().map(|line| {
        parse_inclusive_ranges(line)
    }).collect();

    // println!("input range pairs: {:?}", input_range_pairs);

    let fully_contained_range_pairs: Vec<&(RangeInclusive<i32>, RangeInclusive<i32>)> = input_range_pairs.iter().filter(|(range_a, range_b)| {
        range_fully_contains(range_a, range_b) || range_fully_contains(range_b, range_a)
    }).collect();
    // println!("Fully contained range pairs: {:?}", fully_contained_range_pairs);
    println!("Part 1: Number of fully contained range pairs: {:?}", fully_contained_range_pairs.len());

    let overlapping_range_pairs: Vec<&(RangeInclusive<i32>, RangeInclusive<i32>)> = input_range_pairs.iter().filter(|(range_a, range_b)| {
        ranges_overlap(range_a, range_b)
    }).collect();
    println!("Part 2: Number of overlapping range pairs: {:?}", overlapping_range_pairs.len());
}

fn ranges_overlap(range_a: &RangeInclusive<i32>, range_b: &RangeInclusive<i32>) -> bool {
    let (smaller, larger) = if range_a.end() - range_a.start() < range_b.end() - range_b.start() {
        (range_a, range_b)
    } else {
        (range_b, range_a)
    };
    larger.start() <= smaller.start() && smaller.start() <= larger.end() ||
    larger.start() <= smaller.end() && smaller.end() <= larger.end()
}

fn range_fully_contains(range_a: &RangeInclusive<i32>, range_b: &RangeInclusive<i32>) -> bool {
    range_a.start() <= range_b.start() && range_a.end() >= range_b.end()
}

fn parse_inclusive_ranges(ranges: &str) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let parts: Vec<&str> = ranges.split(",").collect();
    // println!("parse_inclusive_ranges: parts {:?}", parts);
    let range0 = parse_inclusive_range(parts[0]);
    let range1 = parse_inclusive_range(parts[1]);
    (range0, range1)
}

fn parse_inclusive_range(range: &str) -> RangeInclusive<i32> {
    let ends: Vec<&str> = range.split("-").collect();
    let xs: Vec<i32> = ends.iter().map(|x| x.parse::<i32>().unwrap()).collect();
    // println!("xs {:?}", xs);
    RangeInclusive::new(xs[0], xs[1])
}
