#[derive(Debug, Clone, Copy)]
struct Tree {
    size: u8,
}

#[derive(Debug, Clone)]
struct Grid {
    height: usize,
    width: usize,
    trees: Vec<Tree>,
}

impl Grid {
    fn add_tree(&mut self, tree: Tree) {
        self.trees.push(tree);
    }

    fn calculate_2d_index(self, x: usize, y: usize) -> usize {
        self.width * y + x
    }
}

fn main() {
    let rows = include_str!("./input.txt").lines();

    let grid_size = rows.clone().count();
    let mut visible_trees: u16 = 0;
    let mut max_scenic = 0;

    let mut grid: Grid = Grid {
        height: grid_size,
        width: grid_size,
        trees: vec![],
    };

    rows.for_each(|row| {
        row.chars().for_each(|size| {
            let new_tree: &Tree = &Tree {
                size: size.to_digit(10).unwrap() as u8,
            };

            grid.add_tree(new_tree.to_owned());
        });
    });

    for (index, tree) in grid.trees.iter().enumerate() {
        let row_num = index / grid.width;
        let col_num = index % grid.width;
        let mut is_visible = vec![true; 4];
        let mut scores = vec![0; 4];

        //From left
        for pos_x in (0..col_num).rev() {
            let lookup_tree = grid.clone().trees[grid.clone().calculate_2d_index(pos_x, row_num)];
            scores[0] += 1;
            if lookup_tree.size >= tree.size {
                is_visible[0] = false;
                break;
            }
        }

        //From right
        for pos_x in (col_num + 1)..grid.width {
            let lookup_tree = grid.clone().trees[grid.clone().calculate_2d_index(pos_x, row_num)];
            scores[1] += 1;
            if lookup_tree.size >= tree.size {
                is_visible[1] = false;
                break;
            }
        }

        //From top
        for pos_y in (0..row_num).rev() {
            let lookup_tree = grid.clone().trees[grid.clone().calculate_2d_index(col_num, pos_y)];
            scores[2] += 1;
            if lookup_tree.size >= tree.size {
                is_visible[2] = false;
                break;
            }
        }

        //From bottom
        for pos_y in (row_num + 1)..grid.height {
            let lookup_tree = grid.clone().trees[grid.clone().calculate_2d_index(col_num, pos_y)];
            scores[3] += 1;
            if lookup_tree.size >= tree.size {
                is_visible[3] = false;
                break;
            }
        }

        if is_visible.into_iter().any(|visibility| visibility == true) {
            visible_trees += 1;
        }
        let total_score = scores.into_iter().fold(1, |acc, score| acc * score);
        if total_score > max_scenic {
            max_scenic = total_score;
        }
    }

    println!("Part 1: {}", visible_trees);
    println!("Part 2: {}", max_scenic);
}
