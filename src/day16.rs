use crate::day::Day;

use std::error::Error;
use std::io::BufRead;

#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    data: Data,
}

#[derive(Debug)]
enum Data {
    Literal(i64),
    Operator(Vec<Box<Packet>>),
}

impl Packet {
    fn evaluate(&self) -> i64 {
        match &self.data {
            Data::Literal(x) => *x,
            Data::Operator(children) => match &self.type_id {
                0 => children.iter().map(|child| child.evaluate()).sum(),
                1 => children.iter().map(|child| child.evaluate()).product(),
                2 => children.iter().map(|child| child.evaluate()).min().unwrap(),
                3 => children.iter().map(|child| child.evaluate()).max().unwrap(),
                5 => {
                    if children[0].evaluate() > children[1].evaluate() {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if children[0].evaluate() < children[1].evaluate() {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if children[0].evaluate() == children[1].evaluate() {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            },
        }
    }
}

// TODO: generalize with num crate
fn add_bit_u8(v: u8, b: &u8) -> u8 {
    (v << 1) | *b
}

fn add_bit_i64(v: i64, b: &u8) -> i64 {
    (v << 1) | i64::from(*b)
}

fn parse_literal(i: &[u8], acc: i64) -> (&[u8], i64) {
    let chunk = &i[0..5];
    let rem = &i[5..];
    let res = chunk[1..].iter().fold(acc, add_bit_i64);

    if chunk[0] > 0 {
        parse_literal(rem, res)
    } else {
        (&rem, res)
    }
}

// TODO: nom?
fn parse_packet(i: &[u8]) -> (&[u8], Packet) {
    let version = i[0..3].iter().fold(0, add_bit_u8);
    let type_id = i[3..6].iter().fold(0, add_bit_u8);

    let mut i = &i[6..];

    let data = if type_id == 4 {
        let (rem, value) = parse_literal(i, 0);
        i = rem;
        Data::Literal(value)
    } else {
        let len_type_id = i[0];
        i = &i[1..];

        let children = match len_type_id {
            0 => {
                let packet_len = i[..15].iter().fold(0, add_bit_i64) as usize;
                i = &i[15..];

                let mut children = vec![];
                let mut child_slice = &i[..packet_len];
                i = &i[packet_len..];

                while !child_slice.is_empty() {
                    let (rem, packet) = parse_packet(child_slice);
                    child_slice = rem;
                    children.push(Box::new(packet));
                }

                children
            }
            1 => {
                let num_children = i[..11].iter().fold(0, add_bit_i64);
                i = &i[11..];
                let mut children = vec![];
                for _ in 0..num_children {
                    let (rem, packet) = parse_packet(i);
                    i = rem;
                    children.push(Box::new(packet));
                }

                children
            }
            _ => unimplemented!(),
        };

        Data::Operator(children)
    };

    (
        i,
        Packet {
            version: version,
            type_id: type_id,
            data: data,
        },
    )
}

fn parse_data(s: &str) -> Packet {
    let bits: Vec<u8> = s
        .bytes()
        .flat_map(|c| {
            let x = if c <= b'9' { c - b'0' } else { c + 10 - b'A' };
            assert!(x < 0x10);
            let bits = [
                (x & 0x8) >> 3,
                (x & 0x4) >> 2,
                (x & 0x2) >> 1,
                (x & 0x1) >> 0,
            ];

            bits.into_iter()
        })
        .collect();

    let (_, packet) = parse_packet(&bits);

    packet
}

fn version_sum(packet: &Packet) -> i64 {
    (packet.version as i64)
        + match &packet.data {
            Data::Literal(_) => 0,
            Data::Operator(children) => children.iter().map(|child| version_sum(child)).sum(),
        }
}

pub struct Day16 {
    packet: Packet,
}

impl Day for Day16 {
    fn new<R: BufRead>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let mut command = String::new();
        reader.read_line(&mut command)?;

        let packet = parse_data(&command.trim());

        Ok(Self { packet: packet })
    }

    fn part1(&self) -> String {
        version_sum(&self.packet).to_string()
    }

    fn part2(&self) -> String {
        self.packet.evaluate().to_string()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_data() {
        use crate::*;
        use std::io::BufReader;

        let data = include_bytes!("../data_files/day16.txt");
        let mut reader = BufReader::new(&data[..]);

        let day = Day16::new(&mut reader).unwrap();
        assert_eq!(day.part1(), "871");
        assert_eq!(day.part2(), "68703010504");
    }
}
