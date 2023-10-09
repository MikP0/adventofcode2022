use std::collections::HashSet;

type Position = (i16, i16);

fn main() {
    let moves = include_str!("./input.txt").lines();

    let mut tail_positions = HashSet::<Position>::new();

    let mut knots = vec![(11, 15); 10];

    moves.for_each(|movement| {
        let mut split_movement = movement.split_ascii_whitespace();
        let direction = split_movement.next().unwrap().clone();
        let times = split_movement
            .next()
            .unwrap()
            .clone()
            .parse::<u8>()
            .unwrap();

        for _ in 0..times {
            if direction == "L" {
                knots[0].0 -= 1;
            }
            if direction == "R" {
                knots[0].0 += 1;
            }
            if direction == "U" {
                knots[0].1 += 1;
            }
            if direction == "D" {
                knots[0].1 -= 1;
            }
            for index in 0..knots.len() - 1 {
                let head_position = knots[index];
                let tail_position = knots[index + 1];

                let prod = (
                    head_position.0 - tail_position.0,
                    head_position.1 - tail_position.1,
                );

                let mut tail_mod = tail_position.clone();

                if prod == (-2, -2)
                    || prod == (-1, -2)
                    || prod == (-2, -1)
                    || prod == (-2, 0)
                    || prod == (-2, 1)
                    || prod == (-1, 2)
                    || prod == (-2, 2)
                {
                    tail_mod.0 -= 1;
                }

                if prod == (1, 2)
                    || prod == (2, 1)
                    || prod == (2, 0)
                    || prod == (2, -1)
                    || prod == (1, -2)
                    || prod == (2, 2)
                    || prod == (2, -2)
                {
                    tail_mod.0 += 1;
                }

                if prod == (-2, 1)
                    || prod == (-1, 2)
                    || prod == (0, 2)
                    || prod == (1, 2)
                    || prod == (2, 1)
                    || prod == (2, 2)
                    || prod == (-2, 2)
                {
                    tail_mod.1 += 1;
                }

                if prod == (-2, -2)
                    || prod == (2, -1)
                    || prod == (1, -2)
                    || prod == (0, -2)
                    || prod == (-1, -2)
                    || prod == (-2, -1)
                    || prod == (2, -2)
                {
                    tail_mod.1 -= 1;
                }

                knots[index + 1] = tail_mod;
                if index == knots.len() - 2 {
                    tail_positions.insert(tail_mod);
                }
            }
        }
    });

    println!("Part 2: {:?}", tail_positions.len());
}
