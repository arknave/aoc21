# Advent of Code 2021 Solutions

## Intro

[Advent of Code 2021](https://adventofcode.com/2021/) solutions written in Rust. Still trying to get a feel for the language. Goals in rough order are

- Correct answers that make few input assumptions. It's trivial to fail almost all of my solutions by passing in gigabytes of input, but that will fail most solutions. Other assumptions I make on the input that are not guaranteed by the spec are documented in the specific module (see day 17 or 24 for examples).
- Idiomatic code. I tried doing proper error handling at the beginning but quickly gave up.
- No manually editing the input files (must parse them as-is).
- Fast execution. No individual day (both parts) should take more than 5s on my 2018 laptop i5, and most should be much faster.

Initially, I'm not using any crates. I know this is not Rust-like behavior, but I wanted to see how far you can get with the standard library. As it turns out, quite far! I'm planning on going back and adding crates once I've solved all the problems.

## Raw notes / TODOs for me.

Things I want to do better:
- Signum from num crate
- Chain results with `and_then` instead of the current nonsense
- Assert iterator has only one element in day 8 (itertools: `exactly_one`)
- Some of the split parsing would be a lot cleaner with regex.

Ideas:
- Make some kind of common grid + graph system between days 9 and 11? These are two days I'm not happy with.
- 12 as well. I think I need a general graph class
- 15 also uses this grid...
- Abstract out Dijkstra/A\* from 15 + 23

Noted repetition:
- Error code for custom types is really verbose. I think we can fix this by making ParseInputError an io::Error and using `and_then`. Or just enumming the 3 kinds of errors we can get in this project (IO / ParseInt / Custom Parse)
- Parsing over lines seems common (1, 2, 5, 8, ...) and has gross turbofish. Extract out?
- Test harness code is almost identical everywhere. Replace with macro?

Slowness:
- Day 19 is so slow and we can definitely do better. There's a factor of two speedup by figuring out the right hand rule.
- Day 12 is pretty slow. Can help by making that graph class.
- Day 15 isn't fast either. Can speed up using a BFS trick since all edges are low weight? Also could benefit from a smarter grid.
- Day 18 is slow, not sure why. Probably can get rid of some cloning
- Day 20 is also slow :(
- Day 22 is a little slow. Maybe the inclusion exclusion version would be faster?
- Day 23 is a slower than I'd like but I don't have much energy
