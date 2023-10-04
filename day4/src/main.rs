fn main() {
    let lines = include_str!("./input.txt").lines();

    let (sum_of_contains, sum_of_overlaps) = lines.fold(
        (0, 0),
        |(mut contain_acc, mut overlap_acc): (u32, u32), pair: &str| {
            let (elf1, elf2) = pair.split_once(",").unwrap();

            let (elf1_min, elf1_max) = elf1.split_once("-").unwrap();
            let (elf2_min, elf2_max) = elf2.split_once("-").unwrap();

            let (a, b, c, d) = (
                elf1_min.parse::<u32>().unwrap(),
                elf1_max.parse::<u32>().unwrap(),
                elf2_min.parse::<u32>().unwrap(),
                elf2_max.parse::<u32>().unwrap(),
            );

            if (a >= c && b <= d) || (a <= c && b >= d) {
                contain_acc += 1;
            }

            if a <= d && c <= b {
                overlap_acc += 1;
            }

            return (contain_acc, overlap_acc);
        },
    );

    println!("Part1: {} \nPart2: {}", sum_of_contains, sum_of_overlaps);
}
