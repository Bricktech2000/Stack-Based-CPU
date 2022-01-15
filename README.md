# Stack-Based CPU

A custom stack-based CPU architecture

## Dependencies

- [`cargo`](https://crates.io/crates/cargo)

## Running a Sample Program

Execute the following command to run Conway's Game of Life.

```bash
clear; cargo run --release --bin asm src/tests/test15.asm && cargo run --release --bin emu src/tests/test15.asm.bin
```

## Components

### Assembler

`src/bin/asm.rs`

1. Takes a source file (`.asm`) as input
2. Resolves labels, expands hard-coded macros and parses comments
3. Creates a binary file (`.asm.bin`) as output

### Emulator

`src/bin/emu.rs`

1. Takes a binary file (`.asm.bin`) as input
2. Emulates the CPU while printing the display buffer and `stdout` buffer
3. Prints the return value of the program and whether it exited successfully

See [doc/architecture.md](doc/architecture.txt) for documentation of the architecturee.

### Tests

`src/tests/*.asm`

- Various programs that test the feature set of the CPU
