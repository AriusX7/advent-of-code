#[derive(Clone, Copy, Debug, PartialEq)]
enum Position {
    Floor,
    Empty,
    Occupied,
}

#[derive(Clone, Debug, PartialEq)]
struct Layout {
    positions: Vec<Position>,
    width: usize,
    height: usize,
}

impl Layout {
    fn get(&self, x: usize, y: usize) -> Option<&Position> {
        self.positions.get((self.width * y) + (x % self.width))
    }

    // This function can be improved a lot but I'm lazy.
    fn get_adjacent_seats(&self, index: usize, single: bool) -> Vec<&Position> {
        let (x, y) = (index % self.width, index / self.width);

        let mut indices = Vec::new();

        // Left.
        for i in 1.. {
            if x < i {
                break;
            }

            indices.push((x - i, y));

            if single {
                break;
            }
            if let Some(p) = self.get(x - i, y) {
                if !matches!(p, Position::Floor) {
                    break;
                }
            }
        }

        // Right.
        for i in 1.. {
            if x + i == self.width {
                break;
            }

            indices.push((x + i, y));

            if single {
                break;
            }

            if let Some(p) = self.get(x + i, y) {
                if !matches!(p, Position::Floor) {
                    break;
                }
            }
        }

        // Top.
        for i in 1.. {
            if y < i {
                break;
            }

            indices.push((x, y - i));

            if single {
                break;
            }

            if let Some(p) = self.get(x, y - i) {
                if !matches!(p, Position::Floor) {
                    break;
                }
            }
        }

        // Bottom.
        for i in 1.. {
            if y + i == self.height {
                break;
            }

            indices.push((x, y + i));

            if single {
                break;
            }

            if let Some(p) = self.get(x, y + i) {
                if !matches!(p, Position::Floor) {
                    break;
                }
            }
        }

        // Top-left diagonal.
        for i in 1.. {
            if x < i || y < i {
                break;
            }

            indices.push((x - i, y - i));

            if single {
                break;
            }

            if let Some(p) = self.get(x - i, y - i) {
                if !matches!(p, Position::Floor) {
                    break;
                }
            }
        }

        // Top-right diagonal.
        for i in 1.. {
            if x + i == self.width || y < i {
                break;
            }

            indices.push((x + i, y - i));

            if single {
                break;
            }

            if let Some(p) = self.get(x + i, y - i) {
                if !matches!(p, Position::Floor) {
                    break;
                }
            }
        }

        // Bottom-left diagonal.
        for i in 1.. {
            if x < i || y + i == self.height {
                break;
            }

            indices.push((x - i, y + i));

            if single {
                break;
            }

            if let Some(p) = self.get(x - i, y + i) {
                if !matches!(p, Position::Floor) {
                    break;
                }
            }
        }

        // Bottom-right diagonal.
        for i in 1.. {
            if x + i == self.width || y + i == self.height {
                break;
            }

            indices.push((x + i, y + i));

            if single {
                break;
            }

            if let Some(p) = self.get(x + i, y + i) {
                if !matches!(p, Position::Floor) {
                    break;
                }
            }
        }

        let mut seats = Vec::new();
        for (x, y) in indices {
            if let Some(p) = self.get(x, y) {
                seats.push(p);
            }
        }

        seats
    }
}

#[aoc_generator(day11)]
fn get_layout(input: &str) -> Layout {
    let mut height = 0;
    let mut positions = Vec::new();

    for line in input.lines() {
        height += 1;

        for ch in line.chars() {
            if ch == '.' {
                positions.push(Position::Floor);
            } else if ch == 'L' {
                positions.push(Position::Empty);
            } else if ch == '#' {
                positions.push(Position::Occupied);
            }
        }
    }

    let width = positions.len() / height;

    Layout {
        positions,
        height,
        width,
    }
}

fn find_total_occupied(layout: &Layout, single: bool, max_nearby: usize) -> u32 {
    let mut layout = layout.clone();
    let mut is_changing = true;

    while is_changing {
        let mut changes = Vec::new();
        for (index, position) in layout.positions.iter().enumerate() {
            match position {
                Position::Floor => (),
                Position::Empty => {
                    if layout
                        .get_adjacent_seats(index, single)
                        .iter()
                        .all(|&p| p != &Position::Occupied)
                    {
                        changes.push((index, Position::Occupied));
                    }
                }
                Position::Occupied => {
                    if layout
                        .get_adjacent_seats(index, single)
                        .iter()
                        .filter(|&&p| p == &Position::Occupied)
                        .count()
                        >= max_nearby
                    {
                        changes.push((index, Position::Empty));
                    }
                }
            }
        }

        if changes.is_empty() {
            is_changing = false;
        } else {
            for (index, new_position) in changes {
                let position = layout
                    .positions
                    .get_mut(index)
                    .expect("Expected valid position");
                *position = new_position;
            }
        }
    }

    layout.positions.iter().fold(0, |acc, p| {
        if p == &Position::Occupied {
            acc + 1
        } else {
            acc
        }
    })
}

#[aoc(day11, part1)]
fn part_one(layout: &Layout) -> u32 {
    find_total_occupied(layout, true, 4)
}

#[aoc(day11, part2)]
fn part_two(layout: &Layout) -> u32 {
    find_total_occupied(layout, false, 5)
}
