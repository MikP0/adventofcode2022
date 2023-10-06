use std::collections::HashSet;

fn find_marker(chars: &Vec<char>, size: usize) -> Option<usize> {
    let windows = chars.windows(size);

    for (index, window) in windows.enumerate() {
        let mut duplicate_set = HashSet::new();
        window.into_iter().for_each(|el| {
            duplicate_set.insert(el);
        });

        if duplicate_set.len() == size {
            return Some(index + size);
        }
    }
    None
}

fn main() {
    let chars: Vec<char> = include_str!("./input.txt").chars().collect();
    let packets = find_marker(&chars, 4).unwrap();
    println!("Part 1: {}", packets);
    let messages = find_marker(&chars, 14).unwrap();
    println!("Part 2: {}", messages);
}
