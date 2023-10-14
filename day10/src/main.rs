#[derive(Clone, Debug)]
struct Computer {
    value: i32,
    cycle: i32,
    strenghts: Vec<i32>,
    screen: Vec<char>,
}

const SCREEN_WIDTH: u8 = 40;
const SCREEN_HEIGHT: u8 = 6;

impl Computer {
    fn new() -> Computer {
        Computer {
            value: 1,
            cycle: 0,
            strenghts: vec![],
            screen: vec![],
        }
    }

    fn compute_addx(self, addx_value: i32) -> Self {
        let mut comp = self.iterate_cycle().iterate_cycle();
        comp.value += addx_value;
        comp
    }

    fn iterate_cycle(mut self) -> Self {
        self.cycle += 1; // Start cycle

        let line: u8 = (self.cycle as u8 - 1) / SCREEN_WIDTH;

        if (self.value - ((self.cycle - 1) - (SCREEN_WIDTH * line) as i32)).abs() <= 1 {
            self.screen.push('#');
        } else {
            self.screen.push('.');
        }

        if (self.cycle - 20) % 40 == 0 {
            self.strenghts.push(self.value * self.cycle);
        }

        self
    }
}

fn main() {
    let instructions = include_str!("input.txt").lines();

    let mut computer = Computer::new();

    instructions.for_each(|instruction| {
        if instruction.contains("addx") {
            let (_, addx_value) = instruction.split_at(5);
            computer = computer
                .to_owned()
                .compute_addx(addx_value.parse::<i32>().unwrap());
        } else {
            computer = computer.to_owned().iterate_cycle();
        }
    });

    // Part 1
    println!("Part 1: {:?}", computer.strenghts.iter().sum::<i32>());

    // Part 2
    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            print!("{}", computer.screen[(x + y * SCREEN_WIDTH) as usize]);
        }
        print!("\n")
    }
}
