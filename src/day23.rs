use crate::day::Day;

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::error::Error;
use std::io::BufRead;
use std::iter;
use std::str;

// TODO: Use enum instead of u8 for amphipods values
const EMPTY: u8 = 4;
const HALL_LEN: usize = 11;
const NUM_SLOTS: usize = 4;
const POW10: [u64; 4] = [1, 10, 100, 1000];
const SLOT2HALL: [usize; NUM_SLOTS] = [2, 4, 6, 8];

// TODO: const_assert
// const_assert!(usize::from(EMPTY) == NUM_SLOTS);

struct StateInfo {
    /// Depth of a slot
    depth: usize,
    /// Can an amphipod enter this slot (no foreign amphipods)
    can_place: [bool; NUM_SLOTS],
    /// Are all cells between slot and hallways pot clear? Note the *between*
    reach: [[bool; HALL_LEN]; NUM_SLOTS],
    filled: Vec<bool>,
}

fn absdiff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

impl StateInfo {
    fn new(state: &[u8]) -> Self {
        let slot_data = &state[HALL_LEN..];
        let depth = slot_data.len() / NUM_SLOTS;

        let mut can_place = [true; NUM_SLOTS];
        for (idx, &data) in slot_data.iter().enumerate() {
            let exp = idx / depth;
            if data < EMPTY && usize::from(data) != exp {
                can_place[exp] = false;
            }
        }

        let filled: Vec<bool> = state.iter().map(|&x| x != EMPTY).collect();

        let mut reach = [[true; HALL_LEN]; NUM_SLOTS];
        for slot_idx in 0..NUM_SLOTS {
            let tip = SLOT2HALL[slot_idx];
            for hall_idx in (0..tip).rev() {
                reach[slot_idx][hall_idx] = reach[slot_idx][hall_idx + 1] && !filled[hall_idx + 1];
            }

            for hall_idx in (tip + 1)..HALL_LEN {
                reach[slot_idx][hall_idx] = reach[slot_idx][hall_idx - 1] && !filled[hall_idx - 1];
            }
        }

        Self {
            depth,
            can_place,
            reach,
            filled,
        }
    }

    const fn is_hall(idx: usize) -> bool {
        idx < HALL_LEN
    }

    fn get_slot(&self, idx: usize) -> (usize, usize) {
        ((idx - HALL_LEN) / self.depth, (idx - HALL_LEN) % self.depth)
    }

    fn slot_tip(&self, slot: usize) -> usize {
        HALL_LEN + slot * self.depth
    }

    fn slot_tip_for(&self, amphipod: u8) -> usize {
        self.slot_tip(usize::from(amphipod))
    }

    fn top_amphipod(&self, slot: usize) -> usize {
        assert!(!self.can_place[slot]);
        let mut idx = self.slot_tip(slot);
        while !self.filled[idx] {
            idx += 1;
        }

        let (res_slot, _) = self.get_slot(idx);
        assert_eq!(res_slot, slot);

        idx
    }

    fn can_move_to_slot(&self, src: usize, amphipod: u8) -> bool {
        assert!(Self::is_hall(src) && amphipod < EMPTY);

        let slot = usize::from(amphipod);
        self.can_place[slot] && self.reach[slot][src]
    }

    fn can_move_to_hall(&self, slot: usize, hall: usize) -> bool {
        assert!(Self::is_hall(hall) && slot < NUM_SLOTS);

        !SLOT2HALL.contains(&hall) && !self.filled[hall] && self.reach[slot][hall]
    }

    fn get_dest(&self, amphipod: u8) -> usize {
        assert!(self.can_place[usize::from(amphipod)]);

        let mut res = self.slot_tip_for(amphipod) + self.depth - 1;
        while self.filled[res] {
            res -= 1;
        }

        assert!(res >= self.slot_tip_for(amphipod));

        res
    }

    /// Get the distance moved by a
    fn get_dist(&self, amphipod: u8, src: usize, dest: usize) -> u64 {
        assert!(amphipod < EMPTY);
        assert!(
            Self::is_hall(src) && !Self::is_hall(dest),
            "src: {} dest: {}",
            src,
            dest
        );

        let coeff = POW10[usize::from(amphipod)];
        let (slot, depth) = self.get_slot(dest);

        // TODO: abs_diff
        let dist = absdiff(src, SLOT2HALL[slot]) + 1 + depth;

        coeff * (dist as u64)
    }

    fn heuristic(&self, state: &[u8]) -> u64 {
        state
            .iter()
            .enumerate()
            .map(|(idx, &amphipod)| {
                if amphipod == EMPTY {
                    return 0;
                }

                let slot_tip = self.slot_tip_for(amphipod);
                if slot_tip <= idx && idx < slot_tip + self.depth {
                    0
                } else if StateInfo::is_hall(idx) {
                    self.get_dist(amphipod, idx, slot_tip)
                } else {
                    let (slot, _) = self.get_slot(idx);
                    let hall = SLOT2HALL[slot];

                    self.get_dist(amphipod, hall, idx) + self.get_dist(amphipod, hall, slot_tip)
                }
            })
            .sum()
    }
}

fn build_state(slots: &[Vec<u8>; NUM_SLOTS]) -> Vec<u8> {
    iter::repeat(EMPTY)
        .take(HALL_LEN)
        .chain(slots.iter().flat_map(|slot| slot.iter().cloned()))
        .collect()
}

#[allow(dead_code)]
fn print_state(state: &[u8]) -> String {
    let mut translated: Vec<u8> = state
        .iter()
        .map(|c| match c {
            0 => b'A',
            1 => b'B',
            2 => b'C',
            3 => b'D',
            4 => b'.',
            c => *c,
        })
        .collect();

    translated.insert(HALL_LEN, b'|');

    str::from_utf8(&translated).expect("valid utf8").to_string()
}

fn solve(slots: &[Vec<u8>; NUM_SLOTS]) -> u64 {
    let depth = slots[0].len();
    assert!(slots.iter().all(|slot| slot.len() == depth));

    let seed = build_state(slots);
    let seed_info = StateInfo::new(&seed);

    let mut dists: HashMap<Vec<u8>, u64> = HashMap::new();
    dists.insert(seed.clone(), 0);

    let mut heap: BinaryHeap<Reverse<(u64, u64, Vec<u8>)>> = BinaryHeap::new();
    let seed_heuristic = seed_info.heuristic(&seed);
    heap.push(Reverse((seed_heuristic, 0, seed.clone())));

    fn is_done(state_info: &StateInfo, state: &[u8]) -> bool {
        state.iter().take(HALL_LEN).all(|&x| x == EMPTY)
            && (0..NUM_SLOTS).all(|slot| {
                let start = state_info.slot_tip(slot);
                let slice = &state[start..start + state_info.depth];

                slice.iter().all(|&c| usize::from(c) == slot)
            })
    }

    while let Some(Reverse((_, dist, state))) = heap.pop() {
        let state_info = StateInfo::new(&state);
        if is_done(&state_info, &state) {
            return dist;
        }

        if dists.get(&state).unwrap_or(&u64::MAX) != &dist {
            continue;
        }

        // for each hallway spot, see if we can move the amphipod to the correct slot
        let to_slot: Vec<(u64, Vec<u8>)> = (0..HALL_LEN)
            .filter(|&src| state[src] != EMPTY && state_info.can_move_to_slot(src, state[src]))
            .map(|src| {
                let mut next_state: Vec<u8> = state.clone();
                let dest = state_info.get_dest(state[src]);
                next_state[dest] = state[src];
                next_state[src] = EMPTY;

                let weight = state_info.get_dist(state[src], src, dest);

                (dist + weight, next_state)
            })
            .collect();

        // for each slot, try and move the top amphipod in the slot to the hallway.
        let to_hall: Vec<(u64, Vec<u8>)> = (0..NUM_SLOTS)
            .filter(|slot| !state_info.can_place[*slot])
            .flat_map(|slot| {
                let src = state_info.top_amphipod(slot);

                // TODO: remove intermediate allocation
                (0..HALL_LEN)
                    .filter(|&dest| state_info.can_move_to_hall(slot, dest))
                    .map(|dest| {
                        let mut next_state: Vec<u8> = state.clone();
                        next_state[dest] = state[src];
                        next_state[src] = EMPTY;

                        let weight = state_info.get_dist(state[src], dest, src);

                        (dist + weight, next_state)
                    })
                    .collect::<Vec<_>>()
                    .into_iter()
            })
            .collect();

        for (cost, next_state) in to_slot.into_iter().chain(to_hall.into_iter()) {
            let cur_cost = dists.entry(next_state.clone()).or_insert(u64::MAX);

            if cost < *cur_cost {
                *cur_cost = cost;
                heap.push(Reverse((
                    cost + state_info.heuristic(&next_state),
                    cost,
                    next_state,
                )));
            }
        }
    }

    unreachable!()
}

pub struct Day23 {
    slots: [Vec<u8>; NUM_SLOTS],
}

impl Day for Day23 {
    fn new<R: BufRead>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let mut buf = vec![];
        reader.read_to_end(&mut buf)?;

        let slots = [
            vec![buf[31] - b'A', buf[45] - b'A'],
            vec![buf[33] - b'A', buf[47] - b'A'],
            vec![buf[35] - b'A', buf[49] - b'A'],
            vec![buf[37] - b'A', buf[51] - b'A'],
        ];

        Ok(Self { slots })
    }

    fn part1(&self) -> String {
        solve(&self.slots).to_string()
    }

    fn part2(&self) -> String {
        let slots = [
            vec![self.slots[0][0], 3, 3, self.slots[0][1]],
            vec![self.slots[1][0], 2, 1, self.slots[1][1]],
            vec![self.slots[2][0], 1, 0, self.slots[2][1]],
            vec![self.slots[3][0], 0, 2, self.slots[3][1]],
        ];
        solve(&slots).to_string()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_data() {
        use crate::*;
        use std::io::BufReader;

        let data = include_bytes!("../data_files/day23.txt");
        let mut reader = BufReader::new(&data[..]);

        let day = Day23::new(&mut reader).unwrap();
        assert_eq!(day.part1(), "12530");
        assert_eq!(day.part2(), "50492");
    }
}
