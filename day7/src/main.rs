use std::{collections::HashMap, ops::Add};

#[derive(Debug)]
struct Directory {
    id: usize,
    name: String,
    parent_directory: Option<usize>,
}

impl Directory {
    fn new(name: &str, id: usize, parent_directory: Option<usize>) -> Directory {
        Directory {
            id,
            name: String::from(name),
            parent_directory: parent_directory,
        }
    }
}

fn main() {
    let commands = include_str!("./input.txt").lines();

    let mut directory_tree = vec![];
    let mut directory_sizes = HashMap::<usize, u32>::new();
    let mut current_directory_index: usize = 0;
    let mut directory_history: Vec<usize> = vec![];

    directory_tree.push(Directory::new("/", 0, None));
    directory_history.push(0);

    for (index, command) in commands.enumerate() {
        let current_directory = directory_tree.get(current_directory_index).unwrap();

        if index == 0 || command.contains("$ ls") {
            continue;
        } else if command.contains("$ cd") {
            let destination = command.split_ascii_whitespace().last().unwrap();

            if destination == ".." {
                current_directory_index = current_directory.parent_directory.unwrap();
                directory_history.pop();
            } else {
                let destination_index = directory_tree
                    .iter()
                    .find(|dir| {
                        dir.name.eq(destination)
                            && dir.parent_directory.unwrap().eq(&current_directory_index)
                    })
                    .unwrap()
                    .id;

                directory_history.push(destination_index.clone());
                current_directory_index = destination_index;
            }

            continue;
        } else if command.contains("dir") {
            let dir_name = command.split_ascii_whitespace().last().unwrap();
            directory_tree.push(Directory::new(
                dir_name,
                directory_tree.len(),
                Some(current_directory_index),
            ));

            continue;
        } else {
            let file_size = command
                .split_ascii_whitespace()
                .next()
                .unwrap()
                .parse::<u32>()
                .unwrap();

            directory_history.iter().for_each(|hist_dir_index| {
                let parent_dir_size = directory_sizes.get(&hist_dir_index).cloned();
                match parent_dir_size {
                    Some(_) => {
                        directory_sizes.insert(
                            hist_dir_index.clone(),
                            parent_dir_size.unwrap().add(file_size),
                        );
                    }
                    None => {
                        directory_sizes.insert(hist_dir_index.clone(), file_size);
                    }
                }
            })
        }
    }

    let p1_value: u32 = directory_sizes
        .values()
        .filter(|size| size <= &&100000)
        .sum();

    println!("Part 1: {}", p1_value);

    let mut sorted_sizes: Vec<&u32> = directory_sizes.values().collect();
    sorted_sizes.sort();

    let unused_space: u32 = 70000000 - **sorted_sizes.last().unwrap();

    let filtered_sizes: Vec<&u32> = sorted_sizes
        .into_iter()
        .filter(|size| **size + unused_space >= 30000000)
        .collect();

    println!("Part 2: {}", filtered_sizes.iter().next().unwrap());
}
