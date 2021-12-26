use crate::day::Day;
use crate::util::ParseInputError;

use std::error::Error;
use std::fmt;
use std::io::{self, BufRead};
use std::ops;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Snailfish {
    Leaf(i64),
    Pair(Box<Snailfish>, Box<Snailfish>),
}

impl Snailfish {
    fn make_pair(lhs: Self, rhs: Self) -> Self {
        Snailfish::Pair(Box::new(lhs), Box::new(rhs))
    }

    fn is_leaf(&self) -> bool {
        match self {
            Snailfish::Leaf(_) => true,
            Snailfish::Pair(_, _) => false,
        }
    }

    fn parse_str(s: &[u8]) -> Result<(&[u8], Self), ParseInputError> {
        match s[0] {
            b'[' => {
                let (s, lhs) = Self::parse_str(&s[1..])?;

                if s[0] != b',' {
                    return Err(ParseInputError(String::from_utf8_lossy(s).into_owned()));
                }

                let (s, rhs) = Self::parse_str(&s[1..])?;
                if s[0] != b']' {
                    return Err(ParseInputError(String::from_utf8_lossy(s).into_owned()));
                }

                Ok((&s[1..], Snailfish::Pair(Box::new(lhs), Box::new(rhs))))
            }
            _ => {
                let mut k = 0;
                while s[k].is_ascii_digit() {
                    k += 1
                }

                let val = s[..k]
                    .iter()
                    .fold(0, |acc, x| 10 * acc + i64::from(x - b'0'));
                Ok((&s[k..], Snailfish::Leaf(val)))
            }
        }
    }

    fn magnitude(&self) -> i64 {
        match self {
            Snailfish::Leaf(val) => *val,
            Snailfish::Pair(lhs, rhs) => 3 * lhs.magnitude() + 2 * rhs.magnitude(),
        }
    }

    fn add_left(self, value: Option<i64>) -> Self {
        match value {
            None => self,
            Some(x) => match self {
                Snailfish::Leaf(val) => Snailfish::Leaf(val + x),
                Snailfish::Pair(lhs, rhs) => Snailfish::Pair(Box::new(lhs.add_left(value)), rhs),
            },
        }
    }

    fn add_right(self, value: Option<i64>) -> Self {
        match value {
            None => self,
            Some(x) => match self {
                Snailfish::Leaf(val) => Snailfish::Leaf(val + x),
                Snailfish::Pair(lhs, rhs) => Snailfish::Pair(lhs, Box::new(rhs.add_right(value))),
            },
        }
    }

    /// Returns:
    /// (did_explode, new_subtree, add_left, add_right)
    fn explode(self, depth: u32) -> (bool, Snailfish, Option<i64>, Option<i64>) {
        match self {
            Snailfish::Leaf(_) => (false, self, None, None),
            Snailfish::Pair(lhs, rhs) => {
                let does_explode = depth >= 4 && lhs.is_leaf() && rhs.is_leaf();

                if does_explode {
                    (
                        true,
                        Snailfish::Leaf(0),
                        Some(lhs.magnitude()),
                        Some(rhs.magnitude()),
                    )
                } else {
                    let (did_lhs_explode, new_lhs, lhs_left, lhs_right) = lhs.explode(depth + 1);
                    if did_lhs_explode {
                        (
                            true,
                            Snailfish::make_pair(new_lhs, rhs.add_left(lhs_right)),
                            lhs_left,
                            None,
                        )
                    } else {
                        let (did_rhs_explode, new_rhs, rhs_left, rhs_right) =
                            rhs.explode(depth + 1);
                        if did_rhs_explode {
                            (
                                true,
                                Snailfish::make_pair(new_lhs.add_right(rhs_left), new_rhs),
                                None,
                                rhs_right,
                            )
                        } else {
                            (false, Snailfish::make_pair(new_lhs, new_rhs), None, None)
                        }
                    }
                }
            }
        }
    }

    fn split(self) -> (bool, Snailfish) {
        match self {
            Snailfish::Leaf(val) => {
                if val < 10 {
                    (false, self)
                } else {
                    let lo_val = val / 2;
                    let hi_val = val - lo_val;
                    (
                        true,
                        Snailfish::make_pair(Snailfish::Leaf(lo_val), Snailfish::Leaf(hi_val)),
                    )
                }
            }
            Snailfish::Pair(lhs, rhs) => {
                let (did_split, lhs) = lhs.split();

                if did_split {
                    (true, Snailfish::make_pair(lhs, *rhs))
                } else {
                    let (did_split, rhs) = rhs.split();
                    (did_split, Snailfish::make_pair(lhs, rhs))
                }
            }
        }
    }

    fn reduce(self) -> Snailfish {
        let mut res = self;
        loop {
            let (did_explode, explode_res, _, _) = res.explode(0);
            res = explode_res;
            if did_explode {
                continue;
            }

            let (did_split, split_res) = res.split();
            res = split_res;
            if did_split {
                continue;
            } else {
                break;
            }
        }

        res
    }
}

impl fmt::Display for Snailfish {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Snailfish::Leaf(x) => x.fmt(f),
            Snailfish::Pair(lhs, rhs) => write!(f, "[{},{}]", lhs, rhs),
        }
    }
}

impl ops::Add for Snailfish {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Snailfish::Pair(Box::new(self), Box::new(other)).reduce()
    }
}

impl FromStr for Snailfish {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, snailfish) = Self::parse_str(s.as_bytes())?;

        Ok(snailfish)
    }
}

pub struct Day18 {
    snailfish: Vec<Snailfish>,
}

impl Day for Day18 {
    fn new<R: BufRead>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let snailfish: Vec<Snailfish> = reader
            .lines()
            .map(|line_res| line_res.map(|line| line.parse()))
            .collect::<io::Result<Result<_, _>>>()??;

        Ok(Self { snailfish })
    }

    fn part1(&self) -> String {
        let merged = self.snailfish.iter().cloned().reduce(|x, y| x + y).unwrap();
        merged.magnitude().to_string()
    }

    fn part2(&self) -> String {
        // TODO: itertools?
        self.snailfish
            .iter()
            .flat_map(|left_sf| {
                self.snailfish.iter().map(move |right_sf| {
                    if left_sf == right_sf {
                        0
                    } else {
                        (left_sf.clone() + right_sf.clone()).magnitude()
                    }
                })
            })
            .max()
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod snailfish_tests {
    #[test]
    fn test_add1() {
        use crate::day18::*;

        let s1: Snailfish = "[1,2]".parse().unwrap();
        let s2: Snailfish = "[[3,4],5]".parse().unwrap();

        let exp: Snailfish = "[[1,2],[[3,4],5]]".parse().unwrap();
        assert_eq!((s1 + s2), exp);
    }

    #[test]
    fn test_explode() {
        use crate::day18::*;

        let test_cases = vec![
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            (
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            ),
            (
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            ),
        ];

        for (sf, exp) in test_cases {
            let sf: Snailfish = sf.parse().unwrap();
            let exp: Snailfish = exp.parse().unwrap();

            let (_, exploded, _, _) = sf.explode(0);

            assert_eq!(exploded, exp);
        }
    }

    #[test]
    fn test_add2() {
        use crate::day18::*;

        let s1: Snailfish = "[[[[4,3],4],4],[7,[[8,4],9]]]".parse().unwrap();
        let s2: Snailfish = "[1,1]".parse().unwrap();

        let exp: Snailfish = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".parse().unwrap();
        assert_eq!((s1 + s2), exp);
    }

    #[test]
    fn test_list1() {
        use crate::day18::*;

        let data = vec!["[1,1]", "[2,2]", "[3,3]", "[4,4]"];
        let summed: Snailfish = data
            .into_iter()
            .map(|s| s.parse().unwrap())
            .reduce(|x, y| x + y)
            .unwrap();

        let exp: Snailfish = "[[[[1,1],[2,2]],[3,3]],[4,4]]".parse().unwrap();

        assert_eq!(summed, exp);
    }

    #[test]
    fn test_list2() {
        use crate::day18::*;

        let data = vec!["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]"];
        let summed: Snailfish = data
            .into_iter()
            .map(|s| s.parse().unwrap())
            .reduce(|x, y| x + y)
            .unwrap();

        let exp: Snailfish = "[[[[3,0],[5,3]],[4,4]],[5,5]]".parse().unwrap();

        assert_eq!(summed, exp);
    }

    #[test]
    fn test_list3() {
        use crate::day18::*;

        let data = vec!["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]", "[6,6]"];
        let summed: Snailfish = data
            .into_iter()
            .map(|s| s.parse().unwrap())
            .reduce(|x, y| x + y)
            .unwrap();

        let exp: Snailfish = "[[[[5,0],[7,4]],[5,5]],[6,6]]".parse().unwrap();

        assert_eq!(summed, exp);
    }

    #[test]
    fn test_list4() {
        use crate::day18::*;

        let to_add = vec![
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
            "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
            "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
            "[7,[5,[[3,8],[1,4]]]]",
            "[[2,[2,2]],[8,[8,1]]]",
            "[2,9]",
            "[1,[[[9,3],9],[[9,0],[0,7]]]]",
            "[[[5,[7,4]],7],1]",
            "[[[[4,2],2],6],[8,7]]",
        ];

        let exps = vec![
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
            "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]",
            "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]",
            "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]",
            "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]",
            "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]",
            "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]",
            "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]",
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        ];

        let mut sf: Snailfish = to_add[0].parse().unwrap();
        for (to_add, exp) in to_add[1..].into_iter().zip(exps.into_iter()) {
            sf = sf + to_add.parse().unwrap();
            let exp = exp.parse().unwrap();
            assert_eq!(sf, exp);
        }
    }

    #[test]
    fn test_magnitude() {
        use crate::day18::*;

        let data = vec![
            ("[[1,2],[[3,4],5]]", 143),
            ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
            ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
            ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
            ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
            (
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
                3488,
            ),
        ];

        for (sf_desc, exp_mag) in data.into_iter() {
            let sf: Snailfish = sf_desc.parse().unwrap();

            assert_eq!(sf.magnitude(), exp_mag);
        }
    }

    #[test]
    fn test_final() {
        use crate::day18::*;

        let data = vec![
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
            "[[[5,[2,8]],4],[5,[[9,9],0]]]",
            "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
            "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
            "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
            "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
            "[[[[5,4],[7,7]],8],[[8,3],8]]",
            "[[9,3],[[9,9],[6,[4,9]]]]",
            "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
            "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
        ];

        let summed: Snailfish = data
            .into_iter()
            .map(|s| s.parse().unwrap())
            .reduce(|x, y| x + y)
            .unwrap();

        let exp: Snailfish = "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"
            .parse()
            .unwrap();

        assert_eq!(summed, exp);
        assert_eq!(summed.magnitude(), 4140);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_data() {
        use crate::*;
        use std::io::BufReader;

        let data = include_bytes!("../data_files/day18.txt");
        let mut reader = BufReader::new(&data[..]);

        let day = Day18::new(&mut reader).unwrap();
        assert_eq!(day.part1(), "3981");
        assert_eq!(day.part2(), "4687");
    }
}
