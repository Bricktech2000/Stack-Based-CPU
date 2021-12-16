use std::env;
use std::fs;
use std::{thread, time::Duration};
use std::io;
use std::io::prelude::*;

fn main() {
  let const_mem_size: usize = 256;
  let const_delay = 50; // ms
  let const_true = 255;
  let const_false = 0;
  let const_break_lookup: [&str; 5] = [
    "Success.",
    "Error: Ran into invalid instruction.",
    "Error: More than one byte on the stack.",
    "Error: Invalid Operand for instruction.",
    "Error: Didn't encounter halt instruction.",
  ];
  let const_hlt = 0;
  let const_unk = 1;
  let const_stk = 2;
  let const_inv = 3;
  let const_ok = 4;

  let args: Vec<String> = env::args().collect();
  if args.len() != 2 {
    println!("Usage: emu <filename>");
    return;
  }

  println!("Emulating CPU...");

  let in_bytes: Vec<u8> = fs::read(&args[1]).expect("Unable to read file.");
  let mut memory: Vec<u8> = vec![0u8; const_mem_size];
  let mut stack_pointer: u8 = 0;
  let mut instruction_pointer: u8 = 0;
  let mut stdout: String = String::new();

  let mut break_type = const_ok;
  while (instruction_pointer as usize) < in_bytes.len() {
    let in_byte = in_bytes[instruction_pointer as usize];

    let high_2_bits: u8 = (in_byte & 0b11000000) >> 6;
    let low_6_bits: u8 = (in_byte & 0b00111111) >> 0;
    let mnemonic = match high_2_bits {
      0b00 => {
        let op_code: u8 = (in_byte & 0b00111111) >> 0;
        match op_code {
          0x00 => { "nop" },
          0x01 => {
            instruction_pointer += 1;
            let value = in_bytes[instruction_pointer as usize];
            psh(&mut memory, &mut stack_pointer, value);
            "ldv"
          },
          0x02 => {
            // TODO
            break_type = const_hlt;
            "hlt"
          },
          0x08 => {
            let port = pop(&mut memory, &mut stack_pointer);
            let value = pop(&mut memory, &mut stack_pointer);
            if port == 0x00 {
              stdout.push(value as char);
              if const_delay == 0 { // TODO
                write!(io::stdout(), "{}", value as char).unwrap();
                io::stdout().flush().unwrap();
              }
            } else {
              // TODO
              break_type = const_inv;
            }
            "out"
          },
          0x09 => {
            let port = pop(&mut memory, &mut stack_pointer);
            if port == 0x00 {
              let mut line: String = String::new();
              // TODO
              if const_delay != 0 { println!("Program is requesting byte from stdin."); }
              std::io::stdin().read_line(&mut line).unwrap();
              stdout += line.as_str();
              psh(&mut memory, &mut stack_pointer, line.as_bytes()[0]);
            } else {
              // TODO
              break_type = const_inv;
            }
            "iin"
          },

          0x11 => {
            let mut address = pop(&mut memory, &mut stack_pointer);
            let value = get(&mut memory, &mut address);
            psh(&mut memory, &mut stack_pointer, value);
            "lda"
          },
          0x12 => {
            let value = pop(&mut memory, &mut stack_pointer);
            let mut address = pop(&mut memory, &mut stack_pointer);
            set(&mut memory, &mut address, value);
            "sta"
          },
          0x13 => { let value = stack_pointer; psh(&mut memory, &mut stack_pointer, value); "lds" },
          0x14 => { stack_pointer = pop(&mut memory, &mut stack_pointer); "sts" },
          0x15 => { psh(&mut memory, &mut stack_pointer, instruction_pointer + 1); "ldi" },
          0x16 => { instruction_pointer = pop(&mut memory, &mut stack_pointer) - 1; "sti" },
          0x17 => {
            let address = pop(&mut memory, &mut stack_pointer);
            psh(&mut memory, &mut stack_pointer, in_bytes[address as usize]);
            "ldp"
          },
          // 0x18
          0x19 => {
            let value = pop(&mut memory, &mut stack_pointer);
            psh(&mut memory, &mut stack_pointer, value);
            psh(&mut memory, &mut stack_pointer, value);
            "dup"
          },
          0x1A => {
            pop(&mut memory, &mut stack_pointer);
            "drp"
          },
          0x1B => {
            let value1 = pop(&mut memory, &mut stack_pointer);
            let value2 = pop(&mut memory, &mut stack_pointer);
            psh(&mut memory, &mut stack_pointer, value1);
            psh(&mut memory, &mut stack_pointer, value2);
            "swp"
          },

          0x20 => { binary_op(&mut memory, &mut stack_pointer, |a, b| a + b); "add" },
          // 0x21 adc
          0x22 => { binary_op(&mut memory, &mut stack_pointer, |a, b| a - b); "sub" },
          // 0x23 subc
          0x24 => { unary_op(&mut memory, &mut stack_pointer, |a| a + 1); "inc" },
          0x25 => { unary_op(&mut memory, &mut stack_pointer, |a| a - 1); "dec" },
          0x26 => { binary_op(&mut memory, &mut stack_pointer, |a, b| (a < b) as u8 * const_true); "ilt" },
          0x27 => { binary_op(&mut memory, &mut stack_pointer, |a, b| (a > b) as u8 * const_true); "igt" },
          0x28 => { binary_op(&mut memory, &mut stack_pointer, |a, b| (a == b) as u8 * const_true); "ieq" },
          0x29 => { unary_op(&mut memory, &mut stack_pointer, |a| (a == 0) as u8 * const_true); "nez" },
          0x2A => { unary_op(&mut memory, &mut stack_pointer, |a| -(a as i8) as u8); "not" },
          0x2B => { unary_op(&mut memory, &mut stack_pointer, |a| (a as i8).abs() as u8); "abs" },

          0x30 => { unary_op(&mut memory, &mut stack_pointer, |a| !a); "not" },
          0x31 => { binary_op(&mut memory, &mut stack_pointer, |a, b| a | b); "oor" },
          0x32 => { binary_op(&mut memory, &mut stack_pointer, |a, b| a & b); "and" },
          0x33 => { binary_op(&mut memory, &mut stack_pointer, |a, b| a ^ b); "xor" },
          0x34 => { binary_op(&mut memory, &mut stack_pointer, |_a, _b| 0); "xnd" },

          _ => {
            // TODO
            println!("Invalid or Unknown Instruction {:#04x}", in_byte);
            break_type = const_unk;
            "unk" },
          }
        },
        0b01 => {
          let offset = low_6_bits;
          let value = pop(&mut memory, &mut stack_pointer);
          set(&mut memory, &mut (stack_pointer + offset), value);
          "sto" },
        0b10 => {
          let offset = low_6_bits;
          let value = get(&mut memory, &mut (stack_pointer + offset));
          psh(&mut memory, &mut stack_pointer, value);
          "ldo" },
        0b11 => {
          let bit3: u8 = (in_byte & 0b00001000) >> 3;
          let low_3_bits: u8 = (in_byte & 0b00000111) >> 0;
          let low_4_bits: u8 = (in_byte & 0b00001111) >> 0;

          let mid_2_bits: u8 = (in_byte & 0b00110000) >> 4;
          match mid_2_bits {
            0b00 => {
              if bit3 == 0b0 {
                let offset = low_3_bits;
                let condition = pop(&mut memory, &mut stack_pointer);
                if condition == const_true { instruction_pointer += offset; }
                else if condition != const_false {
                  // TODO
                  println!("Invalid Operand for Instruction {:#04x}", in_byte);
                  break_type = const_inv;
                }
                "skp"
              } else {
                // TODO
                println!("Invalid or Unknown Instruction {:#04x}", in_byte);
                break_type = const_unk;
                "unk"
              }
            },
            0b11 => {
              let immediate = low_4_bits;
              psh(&mut memory, &mut stack_pointer, immediate);
              "ldv" },
            _ => {
              // TODO
              println!("Invalid or Unknown Instruction {:#04x}", in_byte);
              break_type = const_unk;
              "unk" },
          }
        },
        _ => {
          // TODO
          println!("Invalid or Unknown Instruction {:#04x}", in_byte);
          break_type = const_unk;
          "unk" },
    };
    if const_delay != 0 {
      println!("stack - instruction: {:02x} - {:02x}", stack_pointer, instruction_pointer);
      println!("op_code = mnemonic:  {:02x} = {}", in_byte, mnemonic);
      println!("memory slice:        {:02x?}", memory.as_slice()[memory.len()-0x0B..].to_vec());
      println!("");
    }

    instruction_pointer += 1;
    if break_type != const_ok { break; }
    thread::sleep(Duration::from_millis(const_delay));
    // _pause();
  }
  if break_type == const_hlt && stack_pointer != -1i8 as u8 { break_type = const_stk; }
  println!("");
  if const_delay != 0 { println!("Standard output:\n{}", stdout); }
  println!("Exit code: {:#04x}, {:#010b} ({}, {})", memory.last().unwrap(), memory.last().unwrap(), memory.last().unwrap(), *memory.last().unwrap() as i8);
  println!("CPU Halted. {}", const_break_lookup[break_type]);
}

fn psh(memory: &mut Vec<u8>, stack_pointer: &mut u8, value: u8) { *stack_pointer -= 1; memory[*stack_pointer as usize] = value; }
fn pop(memory: &mut Vec<u8>, stack_pointer: &mut u8) -> u8 { let temp: u8 = memory[*stack_pointer as usize]; memory[*stack_pointer as usize] = 0; *stack_pointer += 1; temp }
fn set(memory: &mut Vec<u8>, pointer: &mut u8, value: u8) { memory[*pointer as usize] = value; }
fn get(memory: &mut Vec<u8>, pointer: &mut u8) -> u8 { memory[*pointer as usize] }

fn binary_op<F: Fn(u8, u8) -> u8>(memory: &mut Vec<u8>, stack_pointer: &mut u8, closure: F) {
  let operand1 = pop(memory, stack_pointer);
  let operand2 = pop(memory, stack_pointer);
  psh(memory, stack_pointer, closure(operand1, operand2));
}
fn unary_op<F: Fn(u8) -> u8>(memory: &mut Vec<u8>, stack_pointer: &mut u8, closure: F) {
  let operand = pop(memory, stack_pointer);
  psh(memory, stack_pointer, closure(operand));
}

// https://users.rust-lang.org/t/rusts-equivalent-of-cs-system-pause/4494/3
fn _pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}
