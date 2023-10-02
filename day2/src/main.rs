fn main() {
    let mut points: u32 = 0;

    let mut lines = include_str!("input.txt").lines();

    lines.for_each(|round: &str| {
        match round.chars().nth(2).unwrap() {
            'X' => points += 1,
            'Y' => points += 2,
            'Z' => points += 3,
            _ => (),
        }

        match round {
            "A X" | "B Y" | "C Z" => points += 3,
            "C X" | "A Y" | "B Z" => points += 6,
            _ => (),
        }
    });

    println!("Part 1: {}", points);

    points = 0;
    lines = include_str!("input.txt").lines();

    lines.for_each(|round: &str| {
        match round.chars().nth(2).unwrap() {
            'Y' => points += 3,
            'Z' => points += 6,
            _ => (),
        }

        match round {
            "A Y" | "B X" | "C Z" => points += 1, // Rock
            "A Z" | "B Y" | "C X" => points += 2, // Paper
            "A X" | "B Z" | "C Y" => points += 3, // Scissors
            _ => (),
        }
    });

    println!("Part 2: {}", points);
}
