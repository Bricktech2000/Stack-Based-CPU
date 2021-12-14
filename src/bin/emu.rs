use std::env;
use std::fs;

fn main() {
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
  let mut memory: Vec<u8> = vec![0u8; 256];
  let mut stack_pointer: u8 = 255;
  let mut instruction_pointer: u8 = 0;
  let mut load_next: bool = false;

  // consts:
  // 256
  // true and false
  // todo:
  // carry
  // instruction ptr

  for in_byte in in_bytes {
    println!("in_byte: {:x?}", in_byte);
    if load_next == true {
      stack_pointer -= 1; memory[stack_pointer as usize] = in_byte; load_next = false;
      continue;
    }
    // https://doc.rust-lang.org/rust-by-example/primitives/literals.html
    let _src_is_work_pointer: bool = in_byte & 0b10000000u8 != 0;
    let _dst_is_work_pointer: bool = in_byte & 0b01000000u8 != 0;
    let op_code: u8 = in_byte & 0b00111111u8;

    // https://doc.rust-lang.org/rust-by-example/flow_control/match.html
    // https://doc.rust-lang.org/rust-by-example/fn/closures.html
    match op_code {
      0x00 => { /* nop */ },
      0x01 => { load_next = true; },
      0x02 => { break; },

      0x11 => { memory[stack_pointer as usize] = memory[memory[stack_pointer as usize] as usize]; },
      0x12 => { let temp = memory[stack_pointer as usize]; memory[temp as usize] = memory[stack_pointer as usize]; },
      0x13 => { stack_pointer -= 1; memory[stack_pointer as usize] = stack_pointer; },
      0x14 => { stack_pointer = memory[stack_pointer as usize]; stack_pointer += 1; memory[stack_pointer as usize] = 0; },
      0x15 => { stack_pointer -= 1; memory[stack_pointer as usize] = instruction_pointer; },
      0x16 => { instruction_pointer = memory[stack_pointer as usize]; stack_pointer += 1; memory[stack_pointer as usize] = 0; },
      //0x17 => not implemented,
      //0x18 => not implemented,
      //0x19 => not implemented,
      0x19 => { stack_pointer -= 1; memory[stack_pointer as usize] = memory[stack_pointer as usize + 1]; memory[stack_pointer as usize] = 0; },
      0x1A => { stack_pointer += 1; memory[stack_pointer as usize] = 0; },
      0x1B => { stack_pointer -= 1; memory[stack_pointer as usize] = memory[stack_pointer as usize + 2];  stack_pointer -= 1; memory[stack_pointer as usize] = memory[stack_pointer as usize + 2]; },
      0x1C => { stack_pointer += 1; memory[stack_pointer as usize] = 0; stack_pointer += 1; memory[stack_pointer as usize] = 0; },
      0x1D => { let temp = memory[stack_pointer as usize]; memory[stack_pointer as usize] = memory[stack_pointer as usize + 1]; memory[stack_pointer as usize + 1] = temp; },

      0x20 => { memory[stack_pointer as usize + 1] = memory[stack_pointer as usize] + memory[stack_pointer as usize + 1]; stack_pointer += 1; memory[stack_pointer as usize - 1] = 0; },
      //0x21 => not implemented,
      0x22 => { memory[stack_pointer as usize + 1] = memory[stack_pointer as usize] - memory[stack_pointer as usize + 1]; stack_pointer += 1; memory[stack_pointer as usize - 1] = 0; },
      //0x23 => not implemented,
      0x24 => { memory[stack_pointer as usize] += 1 },
      0x25 => { memory[stack_pointer as usize] -= 1 },
      0x26 => { memory[stack_pointer as usize] = if memory[stack_pointer as usize] > 127 { 255 } else { 0 }; },
      0x27 => { memory[stack_pointer as usize] = if memory[stack_pointer as usize] < 127 { 255 } else { 0 }; },
      0x28 => { memory[stack_pointer as usize] = if memory[stack_pointer as usize] != 0 { 255 } else { 0 }; },

      0x30 => { memory[stack_pointer as usize] = !memory[stack_pointer as usize]; },
      0x31 => { memory[stack_pointer as usize + 1] = memory[stack_pointer as usize] | memory[stack_pointer as usize]; stack_pointer += 1; memory[stack_pointer as usize - 1] = 0; },
      0x32 => { memory[stack_pointer as usize + 1] = memory[stack_pointer as usize] & memory[stack_pointer as usize]; stack_pointer += 1; memory[stack_pointer as usize - 1] = 0; },
      0x33 => { memory[stack_pointer as usize + 1] = memory[stack_pointer as usize] ^ memory[stack_pointer as usize]; stack_pointer += 1; memory[stack_pointer as usize - 1] = 0; },
      0x34 => { memory[stack_pointer as usize] = memory[stack_pointer as usize] << 1; },
      0x35 => { memory[stack_pointer as usize] = memory[stack_pointer as usize] >> 1; },
      //0x36 => not implemented,
      //0x37 => not implemented,
      //0x38 => not implemented,
      //0x39 => not implemented,

      _ => { println!("Unknown Instruction {:x?}", in_byte); },
    };
    println!("memory: {:x?}", memory);
    println!("stack_pointer: {}", stack_pointer);
  }
  println!("CPU Halted.");
  println!("memory: {:x?}", memory);
}
