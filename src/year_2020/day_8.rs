#[derive(Clone, Copy, Debug)]
enum Instruction {
    NOP(i32),
    Accumulator(i32),
    Jump(i32),
}

#[aoc_generator(day8)]
fn get_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            if line.starts_with("nop") {
                let nop = line
                    .get(4..)
                    .and_then(|s| s.parse().ok())
                    .expect("Expected valid nop instruction");

                Instruction::NOP(nop)
            } else if line.starts_with("acc") {
                let acc = line
                    .get(4..)
                    .and_then(|s| s.parse().ok())
                    .expect("Expected valid accumulator instruction");

                Instruction::Accumulator(acc)
            } else {
                let jmp = line
                    .get(4..)
                    .and_then(|s| s.parse().ok())
                    .expect("Expected valid jump instruction");

                Instruction::Jump(jmp)
            }
        })
        .collect()
}

#[aoc(day8, part1)]
fn part_one(instructions: &[Instruction]) -> i32 {
    let mut accumulator = 0;

    let mut index = 0;
    let mut visited = Vec::new();

    while !visited.contains(&index) {
        visited.push(index);

        let instruction = instructions.get(index).expect("Expected valid index");

        match instruction {
            Instruction::NOP(_) => index += 1,
            Instruction::Accumulator(acc) => {
                accumulator += *acc;
                index += 1;
            }
            Instruction::Jump(j) => index += *j as usize,
        }
    }

    accumulator
}

fn execute_without_loop(instructions: &[Instruction]) -> Option<i32> {
    let mut accumulator = 0;

    let mut index = 0;
    let mut visited = Vec::new();

    loop {
        if visited.contains(&index) {
            return None;
        }

        visited.push(index);

        if let Some(instruction) = instructions.get(index) {
            match instruction {
                Instruction::NOP(_) => index += 1,
                Instruction::Accumulator(acc) => {
                    accumulator += *acc;
                    index += 1;
                }
                Instruction::Jump(j) => index += *j as usize,
            }
        } else {
            break;
        }
    }

    Some(accumulator)
}

#[aoc(day8, part2)]
fn part_two(instructions: &[Instruction]) -> i32 {
    for i in 0..instructions.len() {
        let mut owned = instructions.to_vec();
        let instruction = &owned[i];
        let new = match instruction {
            Instruction::NOP(n) => Instruction::Jump(*n),
            Instruction::Jump(j) => Instruction::NOP(*j),
            Instruction::Accumulator(_) => continue,
        };

        owned[i] = new;
        if let Some(acc) = execute_without_loop(&owned) {
            return acc;
        } else {
            continue;
        }
    }

    unreachable!()
}
