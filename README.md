# INF8770 - TP1

Repository for the first project of the INF8770 course at Polytechnique Montréal.
It consists in implementing some basic lossless compression algorithms.

## Dependencies and run conditions
It uses Rust as the main programming language, so you need to have it installed in your machine. You can install it by following the instructions at [Rust's official website](https://www.rust-lang.org/learn/get-started).
The program should run with `cargo run --release`. It was tested on Arch Linux with Rust 1.76.0.
The run options are:

`cargo run --release -- [true | false] [rle | lzw]`
- The first argument is a boolean that indicates if the program should run a single instance of the program or a batch of tests.
  - If it is `true`, the program will run a single instance of the algorithm chosen in the second argument. It will display the compression rate and the time taken.
  - If it is `false`, the program will run a batch of tests for the chosen algorithm, 1000 runs for the text data and 100 for the images.
- The second argument is the algorithm to be used. It can be `rle` or `lzw` for the corresponding algorithms.
