use std::fs;

fn main() {
    let input_filename = "input1.txt";

    let contents = fs::read_to_string(input_filename)
        .expect(format!("Unable to read the file \"{input_filename}\"").as_str());

    let elves_input: Vec<&str> = contents.split("\n\n").collect();

    let elves_calories: Vec<Vec<&str>> = elves_input.iter().map(|input| input.split("\n").collect()).collect();
    // for (i, elf_calories) in elves_calories.iter().enumerate() {
    //     println!("Elf #{i} calories:");
    //     for elf_calorie in elf_calories {
    //         println!("    {elf_calorie}");
    //     }
    // }

    let elves_i32_calories: Vec<Vec<i32>> = elves_calories.iter().map(|calories| {
        strings_to_i32s(calories)
    }).collect();

    // for (i, total_calories) in elves_i32_calories.iter().enumerate() {
    //     let elf_number = i + 1;
    //     println!("Elf #{elf_number} calories: {:?}", total_calories);
    // }


    let elves_total_calories: Vec<i32> = elves_i32_calories.iter().map(|cs| sum_calories(cs) ).collect();

    // for (i, total_calories) in elves_total_calories.iter().enumerate() {
    //     let elf_number = i + 1;
    //     println!("Elf #{elf_number} total calories: {:?}", total_calories);
    // }

    let (max_elf, max_calories) = max_elf_calories(&elves_total_calories);
    let elf_number = max_elf + 1;
    println!("Elf #{elf_number} has the most calories {max_calories}.");

    let elves_total_calories2 = elves_calories_without_elf(elves_total_calories, max_elf);
    let (max_elf2, max_calories2) = max_elf_calories(&elves_total_calories2);
    let elf2_number = if max_elf2 >= max_elf { max_elf2 + 2 } else { max_elf2 + 1 };
    println!("Elf #{elf2_number} has the next most calories {max_calories2}.");

    let elves_total_calories3 = elves_calories_without_elf(elves_total_calories2, max_elf2);
    let (max_elf3, max_calories3) = max_elf_calories(&elves_total_calories3);
    let elf3_number = if max_elf3 >= max_elf { max_elf3 + 2 } else { max_elf3 + 1};
    println!("Elf #{elf3_number} has the next most calories {max_calories3}.");

    let total_top_three_calories = max_calories + max_calories2 + max_calories3;
    println!("The total of the top three Elves calories is {total_top_three_calories}.");
}

fn elves_calories_without_elf(elves_total_calories: Vec<i32>, without_elf: usize) -> Vec<i32> {
    elves_total_calories.iter().enumerate().filter(|(i, _)| *i != without_elf ).map(|(_, &cals)|
        cals
    ).collect()
}

fn max_elf_calories(elves_total_calories: &Vec<i32>) -> (usize, i32) {
    elves_total_calories.iter().enumerate().fold((0, 0), |acc, val| {
        let (_acc_i, acc_cal) = acc;
        let (val_i, &val_cal) = val;
        if val_cal > acc_cal { (val_i, val_cal) } else { acc }
    })
}

fn strings_to_i32s(calories: &Vec<&str>) -> Vec<i32> {
    let maybe_values: Vec<i32> = calories.iter().filter_map(|s| {
        match s.parse::<i32>() {
            std::result::Result::Ok(c) => Some(c),
            _ => None,
        }
    }).collect();
    maybe_values
}

fn sum_calories(calories: &Vec<i32>) -> i32 {
    calories.iter().sum()
}
