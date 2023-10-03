fn find_duplicate_char(s1: &str, s2: &str) -> Result<char, ()> {
    for c1 in s1.chars() {
        if s2.chars().any(|c2| c2.eq(&c1)) {
            return Ok(c1);
        }
    }

    return Err(());
}

fn find_duplicate_char_three(s1: &str, s2: &str, s3: &str) -> Result<char, ()> {
    for c1 in s1.chars() {
        if s2.chars().any(|c2| c2.eq(&c1)) && s3.chars().any(|c3| c3.eq(&c1)) {
            return Ok(c1);
        }
    }

    return Err(());
}

fn count_char_priority(c: char) -> u32 {
    if c.is_ascii_uppercase() {
        return c as u32 - 38;
    } else {
        return c as u32 - 96;
    }
}

fn main() {
    let lines = include_str!("./input.txt").lines();

    let part1_priority: u32 = lines.fold(0, |acc: u32, elf: &str| {
        let (pocket1, pocket2) = elf.split_at(elf.len() / 2);

        let duplicate_char: char = find_duplicate_char(pocket1, pocket2).unwrap();

        return acc + count_char_priority(duplicate_char);
    });

    println!("Part 1: {}", part1_priority);

    let lines: Vec<&str> = include_str!("./input.txt").lines().collect();

    let mut part2_priority = 0;

    let _: Vec<&[&str]> = lines
        .windows(3)
        .step_by(3)
        .inspect(|winodw| {
            part2_priority += count_char_priority(
                find_duplicate_char_three(winodw[0], winodw[1], winodw[2]).unwrap(),
            )
        })
        .collect();

    println!("Part 2: {}", part2_priority);
}
