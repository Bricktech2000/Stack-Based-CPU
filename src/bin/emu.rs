use std::env;
use std::fs;
use std::{thread, time::Duration};

fn main() {
  let const_mem_size: usize = 256;
  let const_delay = 50; // ms
  let const_true = 255;
  let const_false = 0;
  let const_break_lookup: [&str; 4] = [
    "Error: Didn't encounter halt instruction.",
    "Error: Ran into invalid instruction.",
    "Error: More than one byte on the stack.",
    "Success.",
  ];
  let const_ext = 0;
  let const_unk = 1;
  let const_stk = 2;
  let const_hlt = 3;

  let args: Vec<String> = env::args().collect();
  if args.len() != 2 {
    println!("Usage: emu <filename>");
    return;
  }

  println!("Emulating CPU...");

  let in_bytes: Vec<u8> = fs::read(&args[1]).expect("Unable to read file.");
  // https://doc.rust-lang.org/std/primitive.array.html
  // https://stackoverflow.com/questions/25805174/creating-a-fixed-size-array-on-heap-in-rust
  // https://stackoverflow.com/questions/46102811/why-i-can-not-use-u8-as-an-index-value-of-a-rust-array
  let mut memory: Vec<u8> = vec![0u8; const_mem_size];
  let mut stack_pointer: u8 = 0;
  let mut instruction_pointer: u8 = 0;

  // todo:
  // carry
  // instruction ptr

  let mut break_type = const_ext;
  while (instruction_pointer as usize) < in_bytes.len() {
    // https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
    // https://users.rust-lang.org/t/reference-cannot-be-written/29894/2
    let in_byte = in_bytes[instruction_pointer as usize];
    let bits76: u8 = (in_byte & 0b11000000) >> 6;
    let bits54: u8 = (in_byte & 0b00110000) >> 4;
    let _bit3: u8 = (in_byte & 0b00001000) >> 3;
    let low_nibble: u8 = (in_byte & 0b00001111) >> 0;
    let op_code: u8 = (in_byte & 0b00111111) >> 0;
    let mut arg1: u8;
    let mut arg2: u8;

    // https://doc.rust-lang.org/rust-by-example/primitives/literals.html

    // https://doc.rust-lang.org/rust-by-example/flow_control/match.html
    // https://doc.rust-lang.org/rust-by-example/fn/closures.html
    let mnemonic = match bits76 {
      0b00 => {
        match op_code {
          0x00 => {
            "nop" },
          0x01 => {
            instruction_pointer += 1;
            arg1 = in_bytes[instruction_pointer as usize];
            push(&mut memory, &mut stack_pointer, arg1);
            "ldv" },
          0x02 => {
            break_type = const_hlt;
            "hlt" },

          0x11 => {
            arg1 = pop(&mut memory, &mut stack_pointer);
            arg2 = get(&mut memory, &mut arg1);
            push(&mut memory, &mut stack_pointer, arg2);
            "lda" },
          0x12 => {
            arg1 = pop(&mut memory, &mut stack_pointer);
            arg2 = pop(&mut memory, &mut stack_pointer);
            set(&mut memory, &mut arg2, arg1);
            "sta" },
          0x13 => {
            arg1 = stack_pointer;
            push(&mut memory, &mut stack_pointer, arg1);
            "ldp" },
          0x14 => {
            arg1 = pop(&mut memory, &mut stack_pointer);
            stack_pointer = arg1;
            "stp" },
          0x15 => {
            arg1 = instruction_pointer + 1;
            push(&mut memory, &mut stack_pointer, arg1);
            "ldi" },
          0x16 => {
            arg1 = pop(&mut memory, &mut stack_pointer);
            instruction_pointer = arg1 + 1;
            "sti" },
          // 0x17 ldc
          // 0x18 stc
          0x19 => {
            arg1 = pop(&mut memory, &mut stack_pointer);
            push(&mut memory, &mut stack_pointer, arg1);
            push(&mut memory, &mut stack_pointer, arg1);
            "dup" },
          0x1A => {
            pop(&mut memory, &mut stack_pointer);
            "drp" },
          0x1B => {
            arg1 = pop(&mut memory, &mut stack_pointer);
            arg2 = pop(&mut memory, &mut stack_pointer);
            push(&mut memory, &mut stack_pointer, arg1);
            push(&mut memory, &mut stack_pointer, arg2);
            "swp" },

          0x20 => {
            arg1 = pop(&mut memory, &mut stack_pointer);
            arg2 = pop(&mut memory, &mut stack_pointer);
            push(&mut memory, &mut stack_pointer, arg1 + arg2);
            "add" },
          // 0x21 adc
          0x22 => {
            arg1 = pop(&mut memory, &mut stack_pointer);
            arg2 = pop(&mut memory, &mut stack_pointer);
            push(&mut memory, &mut stack_pointer, arg1 - arg2);
            "sub" },
          // 0x23 subc
          0x24 => {
            arg1 = pop(&mut memory, &mut stack_pointer);
            arg1 += 1;
            push(&mut memory, &mut stack_pointer, arg1);
            "inc" },
          0x25 => {
            arg1 = pop(&mut memory, &mut stack_pointer);
            arg1 -= 1;
            push(&mut memory, &mut stack_pointer, arg1);
            "dec" },
          0x26 => {
            arg1 = pop(&mut memory, &mut stack_pointer);
            arg2 = pop(&mut memory, &mut stack_pointer);
            push(&mut memory, &mut stack_pointer, if arg1 < arg2 { const_true } else { const_false });
            "ilt" },
          0x27 => {
            arg1 = pop(&mut memory, &mut stack_pointer);
            arg2 = pop(&mut memory, &mut stack_pointer);
            push(&mut memory, &mut stack_pointer, if arg1 > arg2 { const_true } else { const_false });
            "igt" },
          0x28 => {
            arg1 = pop(&mut memory, &mut stack_pointer);
            arg2 = pop(&mut memory, &mut stack_pointer);
            push(&mut memory, &mut stack_pointer, if arg1 == arg2 { const_true } else { const_false });
            "ieq" },
          0x29 => {
            arg1 = pop(&mut memory, &mut stack_pointer);
            arg2 = 0;
            push(&mut memory, &mut stack_pointer, if arg1 != arg2 { const_true } else { const_false });
            "nez" },
          0x2A => {
            arg1 = pop(&mut memory, &mut stack_pointer);
            push(&mut memory, &mut stack_pointer, -(arg1 as i8) as u8);
            "not" },
          0x2B => {
            // https://stackoverflow.com/questions/27182808/how-do-i-get-an-absolute-value-in-rust/55944670
            arg1 = pop(&mut memory, &mut stack_pointer);
            push(&mut memory, &mut stack_pointer, (arg1 as i8).abs() as u8);
            "not" },

          0x30 => {
            arg1 = pop(&mut memory, &mut stack_pointer);
            push(&mut memory, &mut stack_pointer, !arg1);
            "not" },
          0x31 => {
            arg1 = pop(&mut memory, &mut stack_pointer);
            arg2 = pop(&mut memory, &mut stack_pointer);
            push(&mut memory, &mut stack_pointer, arg1 | arg2);
            "oor" },
          0x32 => {
            arg1 = pop(&mut memory, &mut stack_pointer);
            arg2 = pop(&mut memory, &mut stack_pointer);
            push(&mut memory, &mut stack_pointer, arg1 & arg2);
            "and" },
          0x33 => {
            arg1 = pop(&mut memory, &mut stack_pointer);
            arg2 = pop(&mut memory, &mut stack_pointer);
            push(&mut memory, &mut stack_pointer, arg1 ^ arg2);
            "xor" },


          _ => {
            println!("Invalid or Unknown Instruction {:#04x}", in_byte);
            break_type = const_unk;
            "unk" },
          }
        },
        0b01 => {
          arg1 = op_code;
          arg2 = pop(&mut memory, &mut stack_pointer);
          set(&mut memory, &mut (stack_pointer + arg1), arg2);
          "sto"
        },
        0b10 => {
          arg1 = op_code;
          arg2 = get(&mut memory, &mut (stack_pointer + arg1));
          push(&mut memory, &mut stack_pointer, arg2);
          "ldo"
        },
        0b11 => {
          match bits54 {
            0b11 => {
              arg1 = low_nibble;
              push(&mut memory, &mut stack_pointer, arg1);
              "ldv" },
            _ => {
              println!("Invalid or Unknown Instruction {:#04x}", in_byte);
              break_type = const_unk;
              "unk" },
          }
        },
        _ => {
          println!("Internal Error on Instruction {:x?}", in_byte);
          break_type = const_unk;
          "unk" },
    };
    if const_delay != 0 {
      println!("stack - instruction: {:02x} - {:02x}", stack_pointer, instruction_pointer);
      println!("op_code = mnemonic:  {:02x} = {}", in_byte, mnemonic);
      // https://stackoverflow.com/questions/44690439/how-do-i-print-an-integer-in-binary-with-leading-zeros
      // https://stackoverflow.com/questions/44549759/return-last-n-elements-of-vector-in-rust-without-mutating-the-vector
      println!("memory slice:        {:02x?}", memory.as_slice()[memory.len()-0x0B..].to_vec());
      println!("");
    }

    instruction_pointer += 1;
    if break_type > 0 { break; }
    // https://stackoverflow.com/questions/28952938/how-can-i-put-the-current-thread-to-sleep
    thread::sleep(Duration::from_millis(const_delay));
    // _pause();
  }
  if break_type == const_hlt && stack_pointer != -1i8 as u8 { break_type = const_stk; }
  println!("");
  // https://newbedev.com/get-last-element-of-vector-rust-code-example
  println!("Exit code: {:#04x}, {:#010b} ({}, {})", memory.last().unwrap(), memory.last().unwrap(), memory.last().unwrap(), *memory.last().unwrap() as i8);
  println!("CPU Halted. {}", const_break_lookup[break_type]);
}

fn push(memory: &mut Vec<u8>, stack_pointer: &mut u8, value: u8) { *stack_pointer -= 1; memory[*stack_pointer as usize] = value; }
fn pop(memory: &mut Vec<u8>, stack_pointer: &mut u8) -> u8 { let temp: u8 = memory[*stack_pointer as usize]; memory[*stack_pointer as usize] = 0; *stack_pointer += 1; temp }
fn set(memory: &mut Vec<u8>, pointer: &mut u8, value: u8) { memory[*pointer as usize] = value; }
fn get(memory: &mut Vec<u8>, pointer: &mut u8) -> u8 { memory[*pointer as usize] }


// https://users.rust-lang.org/t/rusts-equivalent-of-cs-system-pause/4494/3
use std::io;
use std::io::prelude::*;

fn _pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}
