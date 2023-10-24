#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u128>,
    operation: String,
    test_num: u128,
    true_num: u128,
    false_num: u128,
    inspections: u128,
}

fn main() {
    let lines: Vec<&str> = include_str!("input.txt").lines().collect();

    let mut monkeys: Vec<Monkey> = vec![];

    for window in lines.windows(6).step_by(7) {
        println!("{:?}", window);

        let mut monkey = Monkey {
            test_num: 0,
            items: vec![],
            operation: String::new(),
            false_num: 0,
            true_num: 0,
            inspections: 0,
        };

        // Add items
        window[1]
            .replace(",", "")
            .split_whitespace()
            .for_each(|item| {
                let value = item.parse::<u128>();
                if let Ok(parsed_val) = value {
                    monkey.items.push(parsed_val);
                }
            });

        // Add operation
        monkey.operation = String::from(window[2].replace("Operation: new =", "").trim());

        // Add test num
        monkey.test_num = window[3]
            .replace("Test: divisible by", "")
            .trim()
            .parse::<u128>()
            .unwrap();

        // Add true num
        monkey.true_num = window[4]
            .replace("If true: throw to monkey", "")
            .trim()
            .parse::<u128>()
            .unwrap();

        // Add false num
        monkey.false_num = window[5]
            .replace("If false: throw to monkey", "")
            .trim()
            .parse::<u128>()
            .unwrap();

        monkeys.push(monkey);
    }

    let common_product: u128 = monkeys.iter().map(|monkey| monkey.test_num).product();

    for _ in 0..10000 {
        for monkey_index in 0..monkeys.len() {
            let monkey = &mut monkeys.clone()[monkey_index];
            while let Some(item) = monkey.items.pop() {
                let mut worry = item.clone();

                println!("Worry before: {}", worry);

                let operation_str = monkey.operation.replace("old", &worry.to_string());

                let operation: Vec<&str> = operation_str.split_ascii_whitespace().collect();

                match operation[1] {
                    "+" => {
                        worry = operation[0].parse::<u128>().unwrap()
                            + operation[2].parse::<u128>().unwrap()
                    }
                    "*" => {
                        worry = operation[0].parse::<u128>().unwrap()
                            * operation[2].parse::<u128>().unwrap()
                    }
                    _ => (),
                }

                monkeys[monkey_index].items.pop();

                worry = worry % common_product;

                monkeys[monkey_index].inspections += 1;

                if worry % monkey.test_num == 0 {
                    monkeys[monkey.true_num as usize].items.push(worry);
                } else {
                    monkeys[monkey.false_num as usize].items.push(worry);
                }
            }
        }
    }

    let mut inspections: Vec<u128> = monkeys.iter().map(|monkey| monkey.inspections).collect();

    inspections.sort();

    println!(
        "{}",
        inspections[inspections.len() - 1] * inspections[inspections.len() - 2]
    );
}
