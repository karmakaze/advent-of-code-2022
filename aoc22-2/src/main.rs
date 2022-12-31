use std::fs;
use regex::Regex;

fn main() {
    let input_filename = "input2.txt";

    let contents = fs::read_to_string(input_filename)
        .expect(format!("Unable to read the file \"{input_filename}\"").as_str());

    let game_lines: Vec<&str> = contents.split("\n").collect();

    let re = Regex::new(r"^(.) (.)$").unwrap();
    let game_plays: Vec<(char, char)> = game_lines.iter().flat_map(|line| {
        let game_play: Vec<(char, char)> = re.captures_iter(line).map(|cap| {
            let a = cap[1].chars().next().unwrap();
            let b = cap[2].chars().next().unwrap();
            (a, b)
        }).collect();
        game_play
    }).collect();

    // for (i, game_play) in game_plays.iter().enumerate() {
    //     let (theirs, mine) = game_play;
    //     println!("Game #{i} plays: {theirs} {mine}");
    // }

    let game_scores: Vec<i32> = game_plays.iter().map(|(theirs, mine)| {
        let yo = rps_ordinal(theirs);
        let mo = rps_ordinal(mine);
        let x = (3 + mo - yo) % 3;
        match x {
            0 => mo + 3,
            1 => mo + 6,
            2 => mo,
            _ => 0,
        }
    }).collect();

    // for (i, score) in game_scores.iter().enumerate() {
    //     let number = i + 1;
    //     println!("Game #{number} score: {score}");
    // }

    let total_score: i32 = game_scores.iter().sum();
    println!("Part 1: total score: {total_score}");

    let game_scores2: Vec<i32> = game_plays.iter().map(|(theirs, outcome)| {
        let yo = rps_ordinal(theirs);
        let mo = rps_mine_ordinal(theirs, outcome);
        let x = (3 + mo - yo) % 3;
        match x {
            0 => mo + 3,
            1 => mo + 6,
            2 => mo,
            _ => 0,
        }
    }).collect();
    let total_score2: i32 = game_scores2.iter().sum();
    println!("Part 2: total score: {total_score2}");
}

fn rps_ordinal(c: &char) -> i32 {
    match c {
        &'A' => 1,
        &'B' => 2,
        &'C' => 3,
        &'X' => 1,
        &'Y' => 2,
        &'Z' => 3,
        _ => 0,
    }
}

fn rps_mine_ordinal(theirs: &char, outcome: &char) -> i32 {
    let yo = rps_ordinal(theirs);
    match outcome {
        &'X' => 1 + (yo + 1) % 3,
        &'Y' => yo,
        &'Z' => 1 + yo % 3,
        _ => 0,
    }
}