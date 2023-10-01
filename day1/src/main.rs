use std::{fs::File, io::Read};

fn main() -> std::io::Result<()> {
    let mut file = File::open("src/input.txt")?;
    let mut input = String::new();

    file.read_to_string(&mut input)?;

    let elf_items: Vec<&str> = input.split("\n\n").collect();
    let mut calories: Vec<u32> = elf_items.into_iter().map(|elf| elf.split("\n").map(|calorie_str| calorie_str.parse::<u32>().unwrap()).sum()).collect();
    let max = calories.iter().max().unwrap();

    println!("Part 1: {:?}", max);

    calories.sort();
    calories.reverse();

    let sum_of_top3: u32= calories[0..3].into_iter().sum();

    println!("Part 2: {:?}", sum_of_top3);

    Ok(())
}
