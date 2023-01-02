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

    // for y in 0..lines.len() {
    //     for x in 0..width {
    //         let height = forest.tree_height((x, y));
    //         print!("{height} ");
    //     }
    //     println!();
    // }

    // let mut tallest_height: i32 = -1;
    // let mut tallest_col: usize = 0;
    // let mut tallest_row: usize = 0;
    // forest.visit_trees(|col: usize, row: usize, tree_height: i32| {
    //     if tree_height > tallest_height {
    //         tallest_col = col;
    //         tallest_row = row;
    //         tallest_height = tree_height;
    //     }
    // });
    // println!("tallest tree {tallest_height} at ({tallest_col}, {tallest_row})");

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
