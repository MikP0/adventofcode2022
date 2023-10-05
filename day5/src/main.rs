fn main() {
    let mut lines: Vec<&str> = include_str!("./input.txt").lines().collect();

    let break_line = lines.iter().position(|&line| line.len() == 0).unwrap();
    let (store, manual) = lines.split_at_mut(break_line);

    let column_num = store
        .last()
        .unwrap()
        .trim()
        .chars()
        .last()
        .unwrap()
        .to_string()
        .parse::<usize>()
        .unwrap();

    store.reverse();
    let mut stacks = vec![Vec::<char>::new(); column_num];

    // Fill Stacks
    for store_line in store {
        for (index, element) in store_line.chars().into_iter().enumerate() {
            if (index == 1 || (index as isize - 1) % 4 == 0)
                && element != ' '
                && element.is_alphabetic()
            {
                let stack_num = (index - 1) / 4;
                stacks[stack_num].push(element);
            }
        }
    }

    // Clone Stacks for Part 2
    let mut stacks_p2 = stacks.clone();

    manual[1..manual.len()].iter().for_each(|manual_line| {
        let manual_words: Vec<&str> = manual_line.split(" ").collect();

        let crate_count = manual_words[1].parse::<usize>().unwrap();
        let origin = manual_words[3].parse::<usize>().unwrap() - 1;
        let dest = manual_words[5].parse::<usize>().unwrap() - 1;

        for _ in 0..crate_count {
            let item_to_move = stacks[origin].pop().unwrap();
            stacks[dest].push(item_to_move);
        }

        let mut temp_stack = Vec::<char>::new();

        for _ in 0..crate_count {
            let item_to_move = stacks_p2[origin].pop().unwrap();
            temp_stack.push(item_to_move);
        }

        for _ in 0..crate_count {
            let item_to_move = temp_stack.pop().unwrap();
            stacks_p2[dest].push(item_to_move);
        }
    });

    println!("Part 1:");
    stacks.iter().for_each(|stack| {
        if stack.len() != 0 {
            print!("{}", stack[stack.len() - 1]);
        }
    });
    println!("\n");

    println!("Part 2:");
    stacks_p2.iter().for_each(|stack| {
        if stack.len() != 0 {
            print!("{}", stack[stack.len() - 1]);
        }
    });
}
