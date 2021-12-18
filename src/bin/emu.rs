use std::env;
use std::fs;
use std::{thread, time::Duration};
use std::io;
use std::io::prelude::*;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() != 2 {
    println!("Usage: emu <filename>");
    return;
  }

  println!("Emulating CPU...");

  let in_bytes: Vec<u8> = fs::read(&args[1]).expect("Unable to read file.");
  let exit_code = emulate(in_bytes);

  println!("");
  println!("CPU halted successfully.");
  println!("Exit code: {:#04x}, {:#010b} ({}, {})", exit_code, exit_code, exit_code, exit_code as i8);
}

fn emulate(in_bytes: Vec<u8>) -> u8 {
  let const_mem_size: usize = 256;
  let const_debug = false; // whether to show debug statements
  let const_step = false; // whether to step through the program manually
  let const_true = 255; // value of true
  let const_false = 0; // value of false

  let mut memory: Vec<u8> = vec![0u8; const_mem_size]; // program RAM
  let mut stack_pointer: u8 = 0; // CPU stack pointer
  let mut instruction_pointer: u8 = 0; // CPU instruction pointer
  let mut stdout: String = String::new(); // stdout buffer for debugging

  while (instruction_pointer as usize) < in_bytes.len() {
    let in_byte = in_bytes[instruction_pointer as usize];

    let high_2_bits: u8 = (in_byte & 0b11000000) >> 6;
    let low_6_bits: u8 = (in_byte & 0b00111111) >> 0;
    let mnemonic = match high_2_bits {
      // normal instruction operating on the stack
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
          0x02 => { "hlt"; break; },
          0x08 => {
            let port = pop(&mut memory, &mut stack_pointer);
            let value = pop(&mut memory, &mut stack_pointer);
            if port == 0x00 {
              stdout.push(value as char);
              if !const_debug {
                write!(io::stdout(), "{}", value as char).unwrap();
                io::stdout().flush().unwrap();
              }
            } else { die(0x02, instruction_pointer, port); }
            "out"
          },
          0x09 => {
            let port = pop(&mut memory, &mut stack_pointer);
            if port == 0x00 {
              let mut line: String = String::new();
              if const_debug { println!("Program is requesting byte from stdin."); }
              std::io::stdin().read_line(&mut line).unwrap();
              stdout += line.as_str();
              psh(&mut memory, &mut stack_pointer, line.as_bytes()[0]);
            } else { die(0x02, instruction_pointer, port); }
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

          _ => { die(0x01, instruction_pointer, in_byte); "unk" },
          }
        },
        // instructions operating with a 6-bit operand
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
        // instructions operating with a 3- and 4-bit operand
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
                else if condition != const_false { die(0x03, instruction_pointer, condition); }
                "skp"
              } else { die(0x01, instruction_pointer, in_byte); "unk" }
            },
            0b11 => { let immediate = low_4_bits; psh(&mut memory, &mut stack_pointer, immediate); "ldv" },
            _ => { die(0x01, instruction_pointer, in_byte); "unk" },
          }
        },
        _ => { die(0x01, instruction_pointer, in_byte); "unk" },
    };
    if const_debug {
      println!("stack - instruction: {:02x} - {:02x}", stack_pointer, instruction_pointer);
      println!("op_code = mnemonic:  {:02x} = {}", in_byte, mnemonic);
      println!("stack memory slice   {:02x?}", memory.as_slice()[memory.len()-0x0B..].to_vec());
      println!("Standard output:\n{}", stdout);
      println!("");
    }

    instruction_pointer += 1;

    // delay the execution of the instructions if debug is enabled
    if const_step { pause(); }
    else if const_debug { thread::sleep(Duration::from_millis(50)); }
  }

  // make sure we reached a halt instruction
  if in_bytes[instruction_pointer as usize] != 0x02 { die(0x06, instruction_pointer, 0x00); }
  // make sure only one value is left on the stack
  if stack_pointer != -1i8 as u8 { die(0x05, instruction_pointer, stack_pointer); }
  *memory.last().unwrap()
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

fn die(code: usize, instruction_pointer: u8, value: u8) {
  let message: &str = [
    "Success ",
    "Invalid Instruction: ",
    "Invalid Port: ",
    "Invalid Boolean: ",
    "Invalid Operand: ",
    "Stack does not contain exit code ",
    "Halt instruction was not reached ",
  ][code];

  println!("Fatal Error at {:02x}: {}{:02x}.", instruction_pointer, message, value);
  println!("Exiting.");
  std::process::exit(code as i32);
}

// https://users.rust-lang.org/t/rusts-equivalent-of-cs-system-pause/4494/3
fn pause() {
  let mut stdin = io::stdin();
  let mut stdout = io::stdout();
  // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
  write!(stdout, "Press any key to continue...").unwrap();
  stdout.flush().unwrap();
  // Read a single byte and discard
  let _ = stdin.read(&mut [0u8]).unwrap();
}
