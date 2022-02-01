use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::time::{Duration, Instant};

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
  let mut const_debug = false; // whether to show debug statements
  let mut const_step = false; // whether to step through the program manually
  let const_true = 255; // value of true
  let const_false = 0; // value of false

  let mut memory: Vec<u8> = vec![0u8; const_mem_size]; // program RAM
  let mut display_buffer: Vec<u8> = vec![0u8; const_mem_size]; // display buffer
  let mut stack_pointer: u8 = 0; // CPU stack pointer
  let mut instruction_pointer: u16 = 0; // CPU instruction pointer
  let mut stdout_buffer: String = String::new(); // stdout buffer for debugging
  let mut last_display_or_stdout_update = Instant::now(); // las time the display buffer or stdout was updated

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
            instruction_pointer = safe_instruction_pointer(instruction_pointer, in_bytes.len(), instruction_pointer + 1);
            let value = in_bytes[instruction_pointer as usize];
            psh(&mut memory, &mut stack_pointer, value);
            "xXX"
          },
          0x02 => { "hlt"; break; },
          0x03 => {
            instruction_pointer = safe_instruction_pointer(instruction_pointer, in_bytes.len(), instruction_pointer + 2);
            psh(&mut memory, &mut stack_pointer, in_bytes[instruction_pointer as usize] as u8);
            psh(&mut memory, &mut stack_pointer, in_bytes[instruction_pointer as usize - 1] as u8);
            "xXX xXX"
          },
          0x0F => {
            const_debug = true;
            const_step = true;
            "dbg"
          },

          0x11 => {
            let mut address = pop(&mut memory, &mut stack_pointer);
            let mut value = get(&mut memory, &mut address);
            if address == 0xFF {
              let mut line: String = String::new();
              if const_debug { println!("Program is requesting byte from stdin."); }
              std::io::stdin().read_line(&mut line).unwrap();
              stdout_buffer += line.as_str();
              value = line.as_bytes()[0];
            } else if address == 0xFE {
              // TODO: implement keyboard events
            }
            psh(&mut memory, &mut stack_pointer, value);
            "lda"
          },
          0x12 => {
            let mut address = pop(&mut memory, &mut stack_pointer);
            let value = pop(&mut memory, &mut stack_pointer);
            if address == 0xFF { stdout_buffer.push(value as char); }
            else if address == 0xFE { /* nop */ }
            else { set(&mut memory, &mut address, value); }
            "sta"
          },
          0x13 => { let value = stack_pointer; psh(&mut memory, &mut stack_pointer, value); "lds" },
          0x14 => { stack_pointer = pop(&mut memory, &mut stack_pointer); "sts" },
          0x15 => {
            psh(&mut memory, &mut stack_pointer, ((instruction_pointer + 1) >> 8) as u8);
            psh(&mut memory, &mut stack_pointer, ((instruction_pointer + 1) & 0xFF) as u8);
            "ldi"
          },
          0x16 => {
            let new_ip = (pop(&mut memory, &mut stack_pointer) as u16 | (pop(&mut memory, &mut stack_pointer) as u16) << 8) - 1;
            instruction_pointer = safe_instruction_pointer(instruction_pointer, in_bytes.len(), new_ip);
            "sti"
          },
          0x17 => {
            let address = pop(&mut memory, &mut stack_pointer) as u16 | (pop(&mut memory, &mut stack_pointer) as u16) << 8;
            psh(&mut memory, &mut stack_pointer, in_bytes[address as usize]);
            "ldp"
          },
          0x18 => {
            die(0x08, instruction_pointer, 0x00);
            "stp"
          }
          0x19 => {
            let address = pop(&mut memory, &mut stack_pointer);
            let value = display_buffer[address as usize];
            psh(&mut memory, &mut stack_pointer, value);
            "ldb"
          },
          0x1A => {
            let address = pop(&mut memory, &mut stack_pointer);
            let value = pop(&mut memory, &mut stack_pointer);
            display_buffer[address as usize] = value;
            "stb"
          },
          0x1B => {
            let value = pop(&mut memory, &mut stack_pointer);
            psh(&mut memory, &mut stack_pointer, value);
            psh(&mut memory, &mut stack_pointer, value);
            "dup"
          },
          0x1C => {
            pop(&mut memory, &mut stack_pointer);
            "drp"
          },
          0x1D => {
            let value1 = pop(&mut memory, &mut stack_pointer);
            let value2 = pop(&mut memory, &mut stack_pointer);
            psh(&mut memory, &mut stack_pointer, value1);
            psh(&mut memory, &mut stack_pointer, value2);
            "swp"
          },

          0x20 => { binary_op(&mut memory, &mut stack_pointer, |a, b| u8::wrapping_add(a, b)); "add" },
          0x21 => {
            let operand1 = pop(&mut memory, &mut stack_pointer) as u16;
            let operand2 = pop(&mut memory, &mut stack_pointer) as u16 | (pop(&mut memory, &mut stack_pointer) as u16) << 8;
            let result = operand2 + operand1;
            psh(&mut memory, &mut stack_pointer, (result >> 8) as u8);
            psh(&mut memory, &mut stack_pointer, (result & 0xFF) as u8);
            "adc"
          },
          0x22 => { binary_op(&mut memory, &mut stack_pointer, |a, b| u8::wrapping_sub(a, b)); "sub" },
          0x23 => {
            let operand1 = pop(&mut memory, &mut stack_pointer) as u16;
            let operand2 = pop(&mut memory, &mut stack_pointer) as u16 | (pop(&mut memory, &mut stack_pointer) as u16) << 8;
            let result = operand2 - operand1;
            psh(&mut memory, &mut stack_pointer, (result >> 8) as u8);
            psh(&mut memory, &mut stack_pointer, (result & 0xFF) as u8);
            "sbc"
          },
          0x24 => { unary_op(&mut memory, &mut stack_pointer, |a| u8::wrapping_add(a,  1)); "inc" },
          0x25 => { unary_op(&mut memory, &mut stack_pointer, |a| u8::wrapping_sub(a, 1)); "dec" },
          0x26 => { binary_op(&mut memory, &mut stack_pointer, |a, b| (a < b) as u8 * const_true); "ilt" },
          0x27 => { binary_op(&mut memory, &mut stack_pointer, |a, b| (a > b) as u8 * const_true); "igt" },
          0x28 => { binary_op(&mut memory, &mut stack_pointer, |a, b| (a == b) as u8 * const_true); "ieq" },
          0x29 => { unary_op(&mut memory, &mut stack_pointer, |a| (a != 0) as u8 * const_true); "nez" },
          0x2A => { unary_op(&mut memory, &mut stack_pointer, |a| -(a as i8) as u8); "neg" },
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
              match bit3 {
                0b0 => {
                  let offset = low_3_bits;
                  let condition = pop(&mut memory, &mut stack_pointer);
                  if condition == const_true {
                    instruction_pointer = safe_instruction_pointer(instruction_pointer, in_bytes.len(), instruction_pointer + offset as u16);
                  }
                  else if condition != const_false { die(0x03, instruction_pointer, condition); }
                  "skp"
                },
                _ => { die(0x01, instruction_pointer, in_byte); "unk" },
              }
            },
            0b01 => {
              match bit3 {
                0b0 => {
                  let mut count = low_3_bits;
                  if count == 0x00 { count = pop(&mut memory, &mut stack_pointer); }
                  let value = pop(&mut memory, &mut stack_pointer);
                  psh(&mut memory, &mut stack_pointer, value << count);
                  "shl"
                },
                0b1 => {
                  let mut count = low_3_bits;
                  if count == 0x00 { count = pop(&mut memory, &mut stack_pointer); }
                  let value = pop(&mut memory, &mut stack_pointer);
                  psh(&mut memory, &mut stack_pointer, value >> count);
                  "shr"
                },
                _ => { die(0x01, instruction_pointer, in_byte); "unk" },
              }
            },
            0b10 => {
              match bit3 {
                0b0 => {
                  let mut count = low_3_bits;
                  if count == 0x00 { count = pop(&mut memory, &mut stack_pointer); }
                  let value = pop(&mut memory, &mut stack_pointer);
                  let carry = pop(&mut memory, &mut stack_pointer);
                  psh(&mut memory, &mut stack_pointer, carry + value >> (8 - count));
                  psh(&mut memory, &mut stack_pointer, value << count);
                  "slc"
                },
                0b1 => {
                  let mut count = low_3_bits;
                  if count == 0x00 { count = pop(&mut memory, &mut stack_pointer); }
                  let value = pop(&mut memory, &mut stack_pointer);
                  let carry = pop(&mut memory, &mut stack_pointer);
                  psh(&mut memory, &mut stack_pointer, carry + value << (8 - count));
                  psh(&mut memory, &mut stack_pointer, value >> count);
                  "src"
                },
                _ => { die(0x01, instruction_pointer, in_byte); "unk" },
              }
            },
            // 0b10
            0b11 => { let immediate = low_4_bits; psh(&mut memory, &mut stack_pointer, immediate); "x0X" },
            _ => { die(0x01, instruction_pointer, in_byte); "unk" },
          }
        },
        _ => { die(0x01, instruction_pointer, in_byte); "unk" },
    };
    if last_display_or_stdout_update.elapsed() > Duration::from_millis(1000 / 10) { // ms
      last_display_or_stdout_update = Instant::now();
      print_display_and_stdout(&display_buffer, &stdout_buffer);
    }
    if const_debug {
      println!("stack - instruction: {:02x} - {:04x}", stack_pointer, instruction_pointer);
      println!("op_code = mnemonic:  {:02x} = {}", in_byte, mnemonic);
      println!("stack memory slice   {:02x?}", memory.as_slice()[memory.len()-0x18..].to_vec());
      println!("hex stdout: {:02x?}", stdout_buffer.as_bytes());
      println!("");
    }

    instruction_pointer = safe_instruction_pointer(instruction_pointer, in_bytes.len(), instruction_pointer + 1);

    // delay the execution of the instructions if debug is enabled
    if const_step { pause(); }
    // else if const_debug { thread::sleep(Duration::from_millis(50)); }
    // thread::sleep(Duration::from_micros(10));
  }
  print_display_and_stdout(&display_buffer, &stdout_buffer);


  // make sure we reached a halt instruction
  if in_bytes.len() ==  0 { die(0x06, instruction_pointer, 0x00); }
  if in_bytes[instruction_pointer as usize] != 0x02 { die(0x06, instruction_pointer, 0x00); }
  // make sure only one value is left on the stack
  if stack_pointer != -1i8 as u8 { die(0x05, instruction_pointer, stack_pointer); }
  *memory.last().unwrap()
}

fn psh(memory: &mut Vec<u8>, stack_pointer: &mut u8, value: u8) { *stack_pointer = u8::wrapping_sub(*stack_pointer, 1); memory[*stack_pointer as usize] = value; }
fn pop(memory: &mut Vec<u8>, stack_pointer: &mut u8) -> u8 { let temp: u8 = memory[*stack_pointer as usize]; memory[*stack_pointer as usize] = 0; *stack_pointer = u8::wrapping_add(*stack_pointer, 1); temp }
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

fn safe_instruction_pointer(instruction_pointer: u16, program_memory_length: usize, new_ip: u16) -> u16 {
  if new_ip == program_memory_length as u16 + 1 {
    die(0x06, instruction_pointer, 0x00); 
  } else if new_ip > program_memory_length as u16 {
    die(0x07, instruction_pointer, new_ip as u8); 
  };
  new_ip
}


fn print_display_and_stdout(display_buffer: &Vec<u8>, stdout_buffer: &String) {
  let mut display_buffer_string: String = String::new();
  let line: String = std::iter::repeat("-").take(32).collect::<String>();
  let line_top: String = ".-".to_owned() + &line + "-.\n";
  let line_bottom: String = "'-".to_owned() + &line + "-'\n";
  let col_left: String = "| ".to_string();
  let col_right: String = " |".to_string();

  display_buffer_string += &line_top;
  for y in (0..0x20).step_by(2) {
    display_buffer_string += &col_left;
    for x in 0..0x20 {
      let mut pixel_pair = 0;
      for y2 in 0..2 {
        let address: u8 = (x >> 0x03) | ((y + y2) << 0x02);
        let pixel = display_buffer[address as usize] >> (x & 0x07) & 0x01;
        pixel_pair |= pixel << y2;
      }
      // https://en.wikipedia.org/wiki/Block_Elements
      display_buffer_string += match pixel_pair {
        0b00 => " ",
        0b01 => "\u{2580}",
        0b10 => "\u{2584}",
        0b11 => "\u{2588}",
        _ => "?",
      };
    }
    display_buffer_string += &col_right;
    display_buffer_string.push('\n');
  }
  display_buffer_string += &line_bottom;
  println!("Display buffer:\n{}", display_buffer_string);
  println!("Standard output:\n{}", stdout_buffer);
}

fn die(code: usize, instruction_pointer: u16, value: u8) {
  let message: &str = [
    "Success ",
    "Invalid Instruction: ",
    "Invalid Port: ",
    "Invalid Boolean: ",
    "Invalid Operand: ",
    "Stack does not contain exit code ",
    "Halt instruction was not reached ",
    "Invalid Instruction Pointer: ",
    "Invalid Instruction: stp ",
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
