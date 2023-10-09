use std::collections::HashSet;

type Position = (i16, i16);

fn main() {
    let moves = include_str!("./input.txt").lines();

    let starting_position: Position = (0, 0);
    let mut tail_positions = HashSet::<Position>::new();
    let mut tail_position = starting_position.clone();
    let mut head_position = starting_position.clone();

    let mut knots = vec![(0, 0); 10];

    moves.for_each(|movement| {
        let mut split_movement = movement.split_ascii_whitespace();
        let direction = split_movement.next().unwrap().clone();
        let times = split_movement
            .next()
            .unwrap()
            .clone()
            .parse::<u8>()
            .unwrap();

        println!("{}", direction);

        for _ in 0..times {
            let same_column = tail_position.0 == head_position.0;
            let same_row = tail_position.1 == head_position.1;

            if direction == "L" {
                head_position.0 -= 1;

                let should_move = head_position.0 < (tail_position.0 - 1);
                if same_row && !same_column && should_move {
                    tail_position.0 -= 1;
                }

                if !same_column && !same_row && head_position.0 != tail_position.0 {
                    tail_position = (head_position.0 + 1, head_position.1)
                }
            }

            if direction == "R" {
                head_position.0 += 1;

                let should_move = head_position.0 > (tail_position.0 + 1);
                if same_row && !same_column && should_move {
                    tail_position.0 += 1;
                }

                if !same_column && !same_row && head_position.0 != tail_position.0 {
                    tail_position = (head_position.0 - 1, head_position.1)
                }
            }

            if direction == "U" {
                head_position.1 -= 1;

                let should_move = head_position.1 < (tail_position.1 - 1);
                if same_column && !same_row && should_move {
                    tail_position.1 -= 1;
                }

                if !same_column && !same_row && head_position.1 != tail_position.1 {
                    tail_position = (head_position.0, head_position.1 + 1)
                }
            }

            if direction == "D" {
                head_position.1 += 1;

                let should_move = head_position.1 > (tail_position.1 + 1);
                if same_column && !same_row && should_move {
                    tail_position.1 += 1;
                }

                if !same_column && !same_row && head_position.1 != tail_position.1 {
                    tail_position = (head_position.0, head_position.1 - 1)
                }
            }

            tail_positions.insert(tail_position);
            println!("{:?} {:?}", head_position, tail_position);
        }
    });

    println!("{:?}", tail_positions.len());
}
