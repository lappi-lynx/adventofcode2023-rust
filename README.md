### https://adventofcode.com/2023/

Usage example:
```
cd day-01
cargo build
cargo test --bin part1
cargo test --bin part2
cargo run --profile dhat --features dhat-heap --package day-01 --bin part2
cargo bench

day_01    fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ part1  104.2 µs      │ 134.6 µs      │ 105.9 µs      │ 107.1 µs      │ 100     │ 100
╰─ part2  171.4 µs      │ 262.3 µs      │ 181.1 µs      │ 182.8 µs      │ 100     │ 100

```
