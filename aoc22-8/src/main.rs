use std::fs;
use std::collections::HashMap;

struct Forest {
    width: usize,
    depth: usize,
    trees: Vec<i32>, // tree heights ordered in concatenated rows
}

fn main() {
    let input_filename = "input8.txt";

    let content = fs::read_to_string(input_filename)
        .expect(format!("Unable to read the file \"{input_filename}\"").as_str());
    let input = content.as_str();
    let lines: Vec<&str> = input.split("\n").filter(|line| !line.is_empty()).collect();
    let width = lines.iter().next().unwrap().len();

    let mut forest = Forest { width: width, depth: lines.len(), trees: Vec::new() };
    for line in lines.iter() {
        forest.add_row(line);
    }
    println!("forest width {} x depth {}", forest.width, forest.depth);
    for row in 0..forest.depth {
        for col in 0..forest.width {
            let h = forest.tree_height((col, row));
            print!("{h}");
        }
        println!();
    }

    let mut visible_trees: HashMap<(usize, usize), i32> = HashMap::new();

    for row in 0..forest.depth {
        let mut tallest_visible: i32 = -1;
        for col in 0..forest.width {
            let pos = (col, row);
            let tree_height = forest.tree_height(pos);
            tallest_visible = updater(tallest_visible, tree_height, pos, &mut visible_trees);
        }
        let mut tallest_visible: i32 = -1;
        for col in (0..=(forest.width - 1)).rev() {
            let pos = (col, row);
            let tree_height = forest.tree_height(pos);
            tallest_visible = updater(tallest_visible, tree_height, pos, &mut visible_trees);
        }
    }    

    for col in 0..forest.width {
        let mut tallest_visible: i32 = -1;
        for row in 0..forest.depth {
            let pos = (col, row);
            let tree_height = forest.tree_height(pos);
            tallest_visible = updater(tallest_visible, tree_height, pos, &mut visible_trees);
        }
        let mut tallest_visible: i32 = -1;
        for row in (0..=(forest.depth - 1)).rev() {
            let pos = (col, row);
            let tree_height = forest.tree_height(pos);
            tallest_visible = updater(tallest_visible, tree_height, pos, &mut visible_trees);
        }
    }

    println!("Visible trees:");
    for row in 0..forest.depth {
        for col in 0..forest.width {
            let pos = (col, row);
            let ch = match visible_trees.get(&pos) {
                Some(h) => format!("{}", h),
                None => " ".to_string(),
            };
            print!("{ch}");
        }
        println!();
    }
    println!("Part 1: {} trees are visible from the outside.", visible_trees.len());

    let mut max_score: i32 = -1;
    let mut max_pos = (forest.width, forest.depth);
    for row in 0..forest.depth {
        for col in 0..forest.width {
            let pos = (col, row);
            let score = forest.scenic_score(pos, &mut |_, _| {});
            if score > max_score {
                max_score = score;
                max_pos = pos;
            }
        }
    }
    println!("Part 2: Scenic score of {} visible from {:?}.", max_score, max_pos);

    visible_trees.clear();
    forest.scenic_score(max_pos, &mut |pos, tree_height| {
        visible_trees.insert(pos, tree_height);
    });
    visible_trees.insert(max_pos, forest.tree_height(max_pos));

    println!("Visible trees:");
    for row in 0..forest.depth {
        for col in 0..forest.width {
            let pos = (col, row);
            let ch = match visible_trees.get(&pos) {
                Some(h) => format!("{}", h),
                None => " ".to_string(),
            };
            print!("{ch}");
        }
        println!();
    }
}

fn updater(tallest_visible: i32, tree_height: i32, pos: (usize, usize), mut visible_trees: &mut HashMap<(usize, usize), i32>) -> i32 {
    if tree_height > tallest_visible {
        visible_trees.insert(pos, tree_height);
        return tree_height;
    // } else {
    //     println!("{:?} {} is not visible", pos, tree_height);
    };
    tallest_visible
}

impl Forest {
    fn scenic_score(&self, house_pos: (usize, usize), seen: &mut dyn FnMut((usize, usize), i32) -> ()) -> i32 {
        let mut updater = |house: i32, tree_height: i32, tree_pos: (usize, usize), mut seen_count: &mut i32, mut seen_height: &mut i32| -> bool {
            if tree_height < house {
                *seen_count += 1;
                seen(tree_pos, tree_height);

                if tree_height >= *seen_height {
                    *seen_height = tree_height;
                }
                true
            } else {
                *seen_count += 1;
                seen(tree_pos, tree_height);
                false
            }
        };
        let (house_col, house_row) = house_pos;
        let house_height = self.tree_height(house_pos);
        let mut score: i32 = 1;

        let mut seen_count: i32 = 0;
        let mut seen_height: i32 = -1;
        if house_row > 0 {
            for row in (0..=(house_row - 1)).rev() {
                let tree_pos = (house_col, row);
                let tree_height = self.tree_height(tree_pos);
                if !updater(house_height, tree_height, tree_pos, &mut seen_count, &mut seen_height) {
                    break;
                }
            }
            score *= seen_count;
        }
        if house_pos == (22, 53) {
            println!("Count {seen_count} looking up.");
        }

        seen_count = 0;
        seen_height = -1;
        for row in (house_row + 1)..self.depth {
            let tree_pos = (house_col, row);
            let tree_height = self.tree_height(tree_pos);
            if !updater(house_height, tree_height, tree_pos, &mut seen_count, &mut seen_height) {
                break;
            }
        }
        score *= seen_count;
        if house_pos == (22, 53) {
            println!("Count {seen_count} looking down.");
        }

        if house_col > 0 {
            seen_count = 0;
            seen_height = -1;
            for col in (0..=(house_col - 1)).rev() {
                let tree_pos = (col, house_row);
                let tree_height = self.tree_height(tree_pos);
                if !updater(house_height, tree_height, tree_pos, &mut seen_count, &mut seen_height) {
                    break;
                }
            }
            score *= seen_count;
            if house_pos == (22, 53) {
                println!("Count {seen_count} looking left.");
            }
        }

        seen_count = 0;
        seen_height = -1;
        for col in (house_col + 1)..self.width {
            let tree_pos = (col, house_row);
            let tree_height = self.tree_height(tree_pos);
            if !updater(house_height, tree_height, tree_pos, &mut seen_count, &mut seen_height) {
                break;
            }
        }
        score *= seen_count;
        if house_pos == (22, 53) {
            println!("Count {seen_count} looking right.");
        }

        score
    }

    fn add_row(&mut self, line: &str) {
        for c in line.chars() {
            let height = format!("{c}").parse::<i32>().unwrap();
            self.trees.push(height);
        }
    }

    fn tree_height(&self, pos: (usize, usize)) -> i32 {
        self.trees[self.width * pos.1 + pos.0]
    }

    fn foo<F: Fn(i32) -> i32>(a: i32, f: F) -> i32 {
        f(a)
    }

    fn visit_trees<F: FnMut(usize, usize, i32) -> ()>(&self, mut visitor: F) {
        for (i, tree_height) in self.trees.iter().enumerate() {
            let col = i % self.width;
            let row = i / self.width;
            visitor(col, row, *tree_height);
        }
    }
}
