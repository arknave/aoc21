use crate::day::Day;
use crate::util::Point;

use std::cmp;
use std::error::Error;
use std::io::BufRead;

struct Position {
    pos: Point,
    vel: Point,
}

impl Position {
    fn new(pos: Point, vel: Point) -> Self {
        Self { pos, vel }
    }

    fn step(&self) -> Self {
        assert!(self.vel.x >= 0);
        Self {
            pos: Point::new(self.pos.x + self.vel.x, self.pos.y + self.vel.y),
            vel: Point::new(cmp::max(0, self.vel.x - 1), self.vel.y - 1),
        }
    }
}

/// NOTE: Here I assume the x bounds are always positive, and the y bounds are always negative.
pub struct Day17 {
    x_bounds: (i64, i64),
    y_bounds: (i64, i64),
}

impl Day17 {
    fn in_bounds(&self, p: Point) -> bool {
        self.x_bounds.0 <= p.x
            && p.x <= self.x_bounds.1
            && self.y_bounds.0 <= p.y
            && p.y <= self.y_bounds.1
    }

    fn run_with_vel(&self, vel: Point) -> (bool, i64) {
        let mut pos = Position::new(Point::new(0, 0), vel);

        let mut hit_goal = false;
        let mut max_height = 0;

        while pos.pos.y >= self.y_bounds.0 {
            hit_goal |= self.in_bounds(pos.pos);
            max_height = cmp::max(max_height, pos.pos.y);

            pos = pos.step();
        }

        (hit_goal, max_height)
    }

    fn get_all_heights(&self) -> Vec<i64> {
        (0..=self.x_bounds.1)
            .flat_map(|x_vel| {
                (self.y_bounds.0..=-self.y_bounds.0).filter_map(move |y_vel| {
                    let (hit_goal, max_height) = self.run_with_vel(Point::new(x_vel, y_vel));

                    if hit_goal {
                        Some(max_height)
                    } else {
                        None
                    }
                })
            })
            .collect()
    }
}

impl Day for Day17 {
    fn new<R: BufRead>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        fn parse_range(s: &str) -> (i64, i64) {
            let coords: Vec<i64> = s[2..].split("..").map(|v| v.parse().unwrap()).collect();

            (coords[0], coords[1])
        }

        let mut bounds_desc = String::new();
        reader.read_line(&mut bounds_desc)?;

        let bounds_desc: Vec<&str> = bounds_desc.trim().split(' ').collect();

        let x_bounds = parse_range(bounds_desc[2].trim_end_matches(','));
        let y_bounds = parse_range(bounds_desc[3]);

        assert!(x_bounds.0 < x_bounds.1);
        assert!(y_bounds.0 < y_bounds.1);
        assert!(0 <= x_bounds.0);
        assert!(y_bounds.1 < 0);

        Ok(Self { x_bounds, y_bounds })
    }

    fn part1(&self) -> String {
        self.get_all_heights().iter().max().unwrap().to_string()
    }

    fn part2(&self) -> String {
        self.get_all_heights().len().to_string()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_data() {
        use crate::*;
        use std::io::BufReader;

        let data = include_bytes!("../data_files/day17.txt");
        let mut reader = BufReader::new(&data[..]);

        let day = Day17::new(&mut reader).unwrap();
        assert_eq!(day.part1(), "6786");
        assert_eq!(day.part2(), "2313");
    }
}
