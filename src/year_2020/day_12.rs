struct Instruction {
    action: Action,
    value: u32,
}

enum Action {
    Move(Direction),
    Forward,
    Turn(Turn),
}

enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turn(&mut self, turn: &Turn, value: u32) {
        let times = value / 90;

        for _ in 0..times {
            match turn {
                Turn::Left => self.turn_left(),
                Turn::Right => self.turn_right(),
            }
        }
    }

    fn turn_left(&mut self) {
        match self {
            Self::North => *self = Self::West,
            Self::South => *self = Self::East,
            Self::East => *self = Self::North,
            Self::West => *self = Self::South,
        }
    }

    fn turn_right(&mut self) {
        match self {
            Self::North => *self = Self::East,
            Self::South => *self = Self::West,
            Self::East => *self = Self::South,
            Self::West => *self = Self::North,
        }
    }
}

enum Turn {
    Left,
    Right,
}

#[aoc_generator(day12)]
fn get_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let action_str = line.get(0..1).expect("Expected valid instruction.");
            let value_str = line.get(1..).expect("Expected valid instruction.");

            let action = match action_str {
                "N" => Action::Move(Direction::North),
                "S" => Action::Move(Direction::South),
                "E" => Action::Move(Direction::East),
                "W" => Action::Move(Direction::West),
                "F" => Action::Forward,
                "L" => Action::Turn(Turn::Left),
                "R" => Action::Turn(Turn::Right),
                _ => unreachable!(),
            };

            let value = value_str.parse().expect("Expected valid instruction.");

            Instruction { action, value }
        })
        .collect()
}

fn handle_move(x: &mut i32, y: &mut i32, direction: &Direction, value: u32) {
    match direction {
        Direction::North => *y += value as i32,
        Direction::South => *y -= value as i32,
        Direction::East => *x += value as i32,
        Direction::West => *x -= value as i32,
    }
}

fn handle_turn(direction: &mut Direction, turn: &Turn, value: u32) {
    direction.turn(turn, value);
}

#[aoc(day12, part1)]
fn part_one(instructions: &[Instruction]) -> u32 {
    let mut direction = Direction::East;

    // x-coord, positive means east, negative means west.
    let mut x = 0;
    // y-coord, positive means north, negative means south.
    let mut y = 0;

    for instruction in instructions {
        match &instruction.action {
            Action::Move(dir) => handle_move(&mut x, &mut y, &dir, instruction.value),
            Action::Forward => handle_move(&mut x, &mut y, &direction, instruction.value),
            Action::Turn(turn) => handle_turn(&mut direction, turn, instruction.value),
        }
    }

    (x.abs() + y.abs()) as u32
}

#[aoc(day12, part2)]
fn part_two(instructions: &[Instruction]) -> u32 {
    // coords; first is x--coord, second is y--coord; positive x is east, positive y is north
    let mut ship = (0, 0);
    let mut waypoint = (10, 1);

    for instruction in instructions {
        match &instruction.action {
            Action::Move(dir) => {
                handle_move(&mut waypoint.0, &mut waypoint.1, &dir, instruction.value)
            }
            Action::Forward => {
                let value = instruction.value as i32;

                let (add_x, add_y) = (value * waypoint.0, value * waypoint.1);

                ship.0 += add_x;
                ship.1 += add_y;
            }
            Action::Turn(turn) => {
                let mut value = instruction.value as f32;
                let curr = waypoint;

                if let Turn::Left = turn {
                    value = -value;
                }

                let (sin, cos) = value.to_radians().sin_cos();
                let sin = sin.round() as i32;
                let cos = cos.round() as i32;

                waypoint.0 = curr.0 * cos + curr.1 * sin;
                waypoint.1 = -curr.0 * sin + curr.1 * cos;
            }
        }
    }

    (ship.0.abs() + ship.1.abs()) as u32
}

#[cfg(test)]
mod test_day_12 {
    use super::*;

    #[test]
    fn test_part_two() {
        let input = "F10\nN3\nF7\nR90\nF11";
        let instructions = get_instructions(input);

        assert_eq!(286, part_two(&instructions));
    }
}
