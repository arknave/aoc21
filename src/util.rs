use std::error::Error;
use std::fmt;
use std::ops;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct ParseInputError(pub String);

impl fmt::Display for ParseInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Error for ParseInputError {}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x: x, y: y }
    }
}

impl FromStr for Point {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: simplify with itertools
        let xy: Vec<&str> = s.split(',').collect();

        assert!(xy.len() == 2);
        let x = xy[0].parse().map_err(|_e| ParseInputError(s.to_string()))?;
        let y = xy[1].parse().map_err(|_e| ParseInputError(s.to_string()))?;

        Ok(Point { x: x, y: y })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Point3D {
    coords: [i64; 3],
}

impl Point3D {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { coords: [x, y, z] }
    }

    pub fn transform(&self, perm: [usize; 3], flips: [i64; 3]) -> Self {
        let mut res = [0; 3];
        for idx in 0..3 {
            res[idx] = self.coords[perm[idx]] * flips[idx];
        }

        Self { coords: res }
    }

    pub fn manhattan_dist(&self, other: Self) -> i64 {
        self.coords
            .iter()
            .zip(other.coords.iter())
            .map(|(x, y)| (x - y).abs())
            .sum()
    }
}

impl ops::Add for Point3D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            coords: [
                self.coords[0] + other.coords[0],
                self.coords[1] + other.coords[1],
                self.coords[2] + other.coords[2],
            ],
        }
    }
}

impl ops::Sub for Point3D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            coords: [
                self.coords[0] - other.coords[0],
                self.coords[1] - other.coords[1],
                self.coords[2] - other.coords[2],
            ],
        }
    }
}

impl FromStr for Point3D {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: simplify with itertools
        let xyz: Vec<&str> = s.split(',').collect();

        assert!(xyz.len() == 3);
        let x = xyz[0]
            .parse()
            .map_err(|_e| ParseInputError(s.to_string()))?;
        let y = xyz[1]
            .parse()
            .map_err(|_e| ParseInputError(s.to_string()))?;
        let z = xyz[2]
            .parse()
            .map_err(|_e| ParseInputError(s.to_string()))?;

        Ok(Self::new(x, y, z))
    }
}
