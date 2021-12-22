
use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct ParseInputError(String);

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

impl FromStr for Point {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let xy: Vec<&str> = s.split(',').collect();

        assert!(xy.len() == 2);
        let x = xy[0].parse().map_err(|_e| ParseInputError(s.to_string()))?;
        let y = xy[1].parse().map_err(|_e| ParseInputError(s.to_string()))?;

        Ok(Point { x: x, y: y })
    }
}
