# 🌟 Advent of Code 2020

My solutions for [Advent of Code 2020](https://adventofcode.com/2020)
in [Rust](https://www.rust-lang.org/).

## Running the code

> ⚠️ My input files are not
> committed [as per request](https://www.reddit.com/r/adventofcode/comments/zh2hk0/2022friendly_reminder_dont_commit_your_input/)
> of the Advent of Code author.

1. Make sure [Rust](https://www.rust-lang.org/tools/install) is installed on the system.
2. Add the input in a file named `input.txt` next to the solution before running.
3. Run: `cargo run --bin {day}`

### Faster!

Rust is _**blazingly fast**_, but `cargo run` builds and runs an unoptimized version to improve
compile times at the expense of runtime speed. To achieve ludicrous speed:

Run a release build:

```bash
cargo build -r
```

Then:

```bash
./target/release/{day}
```

Enjoy!