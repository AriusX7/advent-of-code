#[derive(Debug, PartialEq)]
enum Point {
    Open,
    Tree,
}

#[derive(Debug, PartialEq)]
struct Map {
    points: Vec<Point>,
    width: usize,
    height: usize,
}

impl Map {
    fn get(&self, x: usize, y: usize) -> Option<&Point> {
        self.points.get((self.width * y) + (x % self.width))
    }
}

#[aoc_generator(day3)]
fn generate_map(input: &str) -> Map {
    let mut height = 0;
    let mut points = Vec::new();

    for line in input.lines() {
        height += 1;

        for ch in line.chars() {
            if ch == '.' {
                points.push(Point::Open)
            } else {
                points.push(Point::Tree)
            }
        }
    }

    let width = points.len() / height;

    Map {
        points,
        height,
        width,
    }
}

fn count_trees(map: &Map, right: usize, down: usize) -> u64 {
    let mut pos = (0, 0);
    let mut total = 0;

    while pos.1 < map.height {
        pos.0 += right;
        pos.1 += down;

        if let Some(point) = map.get(pos.0, pos.1) {
            if let Point::Tree = point {
                total += 1;
            }
        }
    }

    total
}

#[aoc(day3, part1)]
fn part_one(map: &Map) -> u64 {
    count_trees(map, 3, 1)
}

#[aoc(day3, part2)]
fn part_two(map: &Map) -> u64 {
    count_trees(map, 1, 1)
        * count_trees(map, 3, 1)
        * count_trees(map, 5, 1)
        * count_trees(map, 7, 1)
        * count_trees(map, 1, 2)
}

#[cfg(test)]
mod test_day_3 {
    use super::*;

    fn get_input() -> &'static str {
        concat!(
            "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.",
            "\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#",
            "\n.#..#...#.#\n"
        )
    }

    #[test]
    fn test_get() {
        let map = generate_map(get_input());

        assert_eq!(map.get(12, 4), Some(&Point::Tree))
    }

    #[test]
    fn test_generate_map() {
        assert_eq!(
            generate_map(".#.#.#\n#..#.#"),
            Map {
                points: vec![
                    Point::Open,
                    Point::Tree,
                    Point::Open,
                    Point::Tree,
                    Point::Open,
                    Point::Tree,
                    Point::Tree,
                    Point::Open,
                    Point::Open,
                    Point::Tree,
                    Point::Open,
                    Point::Tree,
                ],
                height: 2,
                width: 6
            }
        )
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&generate_map(get_input())), 7)
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&generate_map(get_input())), 336)
    }
}
