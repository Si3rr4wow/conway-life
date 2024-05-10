# Conway's Game of Life

An implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway's_Game_of_Life), written in Rust. 

## Getting Started

To run in development mode
```
cargo run
```
To build a release version
```
cargo build --release
```
see `targets/release` for output.

## Author's Note

I originally wrote the logic around cells as an exercise to get more familiar with the language. One can see in early commits a rudimentary function for printing the cell's values in the terminal. I'd thought I might work on a way to output to a browser window but then I discovered [winit](https://docs.rs/winit/latest/winit/) and [softbuffer](https://docs.rs/softbuffer/latest/softbuffer/). Together they let me create a window and draw the cells in it. 

There's almost certainly some suboptimal operations in here that I'd like to come and clean up. I'd also like to revisit the event event loop code as it's getting a little long and nested. 