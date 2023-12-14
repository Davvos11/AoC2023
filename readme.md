# Advent of Code 2023
#### By Davvos11, in Rust

You can use cargo to run a specific day (or all days):

```shell
# Run a specific day:
cargo run -- 1
# Run the latest day:
cargo run -- 0
# Run all DAYS:
cargo run -- a
# Get a text prompt to specify a day:
cargo run 
```

Make sure to download your puzzle inputs and put them in the `static` folder, named `input01.txt` (for day 1, for example).

**Note:** this repo is named `AoC2023` (because I did not think this through) but Rust will give warnings if your crate is
named in CamelCase. I renamed the crate to `aoc_2023`, so you have to rename the folder that this repo is cloned in
to `aoc_2023` as well, otherwise it will not work.