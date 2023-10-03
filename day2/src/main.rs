fn main() {
    let mut lines = include_str!("input.txt").lines();

    let mut points = lines.fold(0, |mut sum: u32, round: &str| {
        match round.chars().nth(2).unwrap() {
            'X' => sum += 1,
            'Y' => sum += 2,
            'Z' => sum += 3,
            _ => (),
        }

        match round {
            "A X" | "B Y" | "C Z" => sum += 3,
            "C X" | "A Y" | "B Z" => sum += 6,
            _ => (),
        }

        sum
    });

    println!("Part 1: {}", points);

    lines = include_str!("input.txt").lines();

    points = lines.fold(0, |mut sum: u32, round: &str| {
        match round.chars().nth(2).unwrap() {
            'Y' => sum += 3,
            'Z' => sum += 6,
            _ => (),
        }

        match round {
            "A Y" | "B X" | "C Z" => sum += 1, // Rock
            "A Z" | "B Y" | "C X" => sum += 2, // Paper
            "A X" | "B Z" | "C Y" => sum += 3, // Scissors
            _ => (),
        }

        sum
    });

    println!("Part 2: {}", points);
}
