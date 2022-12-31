use std::fs;
use regex::Regex;

fn main() {
    let input_filename = "input3.txt";

    let contents = fs::read_to_string(input_filename)
        .expect(format!("Unable to read the file \"{input_filename}\"").as_str());

    let rucksack_lines: Vec<&str> = contents.split("\n").filter(|s| !s.is_empty()).collect();

    let rucksack_contents: Vec<(&str, &str)> = rucksack_lines.iter().map(|line| {
        line.split_at(line.chars().count()/2)
    }).collect();

    // for (i, content) in rucksack_contents.iter().enumerate() {
    //     let number = i + 1;
    //     let (x, y) = content;
    //     println!("Rucksack #{number} contains {:?} and {:?}", x, y);
    // }

    let dups: Vec<char> = rucksack_contents.iter().map(|(xs, ys)| {
        let re = Regex::new(format!("[{xs}]").as_str()).unwrap();
        let x = re.find_iter(ys).next().unwrap();
        // println!("x {:?}", x);
        x.as_str().chars().next().unwrap()
    }).collect();

    // for (i, dup) in dups.iter().enumerate() {
    //     let number = i + 1;
    //     println!("Case #{number} duplicated '{dup}'");
    // }

    let priorities: Vec<i32> = dups.iter().map(priority).collect();
    // println!("Dup priorities {:?}", priorities);

    let sum_priorities: i32 = priorities.iter().sum();
    println!("Part 1: Duplicate priorities sum {sum_priorities}");
    println!();

    let grouped_lines: Vec<&[&str]> = rucksack_lines.chunks(3).collect();
    // for (i, group) in grouped_lines.iter().enumerate() {
    //     println!("Group #{:?} {:?}", i + 1, group);
    // }

    let group_badges: Vec<char> = grouped_lines.iter().map(|lines| {
        let xs = lines[0];
        let ys = lines[1];
        let zs = lines[2];
        let re = Regex::new(format!("[{:?}]", ys).as_str()).unwrap();
        let xy_strs: Vec<&str> = re.find_iter(xs).map(|x| x.as_str()).collect();
        let xys = xy_strs.join("");
        // println!("xys {:?}", xys);

        let re = Regex::new(format!("[{:?}]", zs).as_str()).unwrap();
        let xyz_strs: Vec<&str> = re.find_iter(xys.as_str()).map(|xy| xy.as_str()).collect();
        let xyzs = xyz_strs.join("");
        // println!("xyzs {:?}", xyzs);

        xyzs.chars().next().unwrap()
    }).collect();
    // println!("Group badges {:?}", group_badges);

    let group_badge_priorties: Vec<i32> = group_badges.iter().map(priority).collect();
    // println!("Group priorities {:?}", group_badge_priorties);

    let sum_group_badge_priorties: i32 = group_badge_priorties.iter().sum();
    println!("Part 2: Group badge priorities sum {sum_group_badge_priorties}");
}

fn priority(item: &char) -> i32 {
    if 'a' <= *item && *item <= 'z' {
        1 + *item as i32 - 'a' as i32
    } else if 'A' <= *item && *item <= 'Z' {
        27 + *item as i32 - 'A' as i32
    } else {
        0
    }
}
