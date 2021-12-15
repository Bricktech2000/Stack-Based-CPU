use std::env;
use std::fs;
use std::{thread, time::Duration};

fn main() {
  let const_mem_size: usize = 256;
  let const_delay = 50; // ms
  let const_true = 255;
  let const_false = 0;
  let const_break_lookup: [&str; 3] = [
    "Error: reached end of binary without halt.",
    "Error: Ran into invalid instruction.",
    "Successful.",
  ];
  let const_ext = 0;
  let const_unk = 1;
  let const_hlt = 2;

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
  let mut p: [u8; 3] = [0, 0, 0];
  let const_stack: usize = 0;
  let const_work: usize = 1;
  let const_instruction: usize = 2;

  // todo:
  // carry
  // instruction ptr

  let mut load_next: bool = false;
  let mut break_type = const_ext;
  while (p[const_instruction] as usize) < in_bytes.len() {
    // https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
    // https://users.rust-lang.org/t/reference-cannot-be-written/29894/2
    let in_byte = in_bytes[p[const_instruction] as usize];
    let op_code: u8 = in_byte & 0b00111111u8;
    let prefix = (in_byte & 0b11000000) >> 6;
    let mut mnemonic: &str = "unk";
    let src_pointer: usize = if in_byte & 0b10000000u8 != 0 { const_work } else { const_stack };
    let dst_pointer: usize = if in_byte & 0b01000000u8 != 0 { const_work } else { const_stack };

    if load_next == true {
      p[const_stack] -= 1; memory[p[const_stack] as usize] = in_byte; load_next = false;
    } else {
      // https://doc.rust-lang.org/rust-by-example/primitives/literals.html

      // https://doc.rust-lang.org/rust-by-example/flow_control/match.html
      // https://doc.rust-lang.org/rust-by-example/fn/closures.html
      mnemonic = match op_code {
        0x00 => { "nop" },
        0x01 => { load_next = true; "val" },
        0x02 => { break_type = const_hlt; "hlt" },

        0x11 => { memory[p[dst_pointer] as usize] = memory[memory[p[src_pointer] as usize] as usize]; "lda" },
        0x12 => { let temp = memory[p[dst_pointer] as usize]; memory[temp as usize] = memory[p[src_pointer] as usize]; "sta" },
        0x13 => { let temp = p[src_pointer]; p[dst_pointer] -= 1; memory[p[dst_pointer] as usize] = temp; "ldp" },
        0x14 => { p[dst_pointer] = memory[p[src_pointer] as usize]; memory[p[src_pointer] as usize] = 0; p[src_pointer] += 1; "stp" },
        0x15 => { let temp = p[const_instruction] + 1; p[dst_pointer] -= 1; memory[p[dst_pointer] as usize] = temp; "ldi" },
        0x16 => { p[const_instruction] = memory[p[src_pointer] as usize]; memory[p[src_pointer] as usize] = 0; p[src_pointer] += 1; "sti" },
        //0x17 => not implemented, "ldc"
        //0x18 => not implemented, "stc"
        0x19 => { let temp = memory[p[src_pointer] as usize]; p[dst_pointer] -= 1; memory[p[dst_pointer] as usize] = temp; "dup" },
        0x1A => { memory[p[dst_pointer] as usize] = 0; p[dst_pointer] += 1; "drp" },
        0x1B => { p[dst_pointer] -= 1; memory[p[dst_pointer] as usize] = memory[p[src_pointer] as usize + 2];  p[dst_pointer] -= 1; memory[p[dst_pointer] as usize] = memory[p[src_pointer] as usize + 2]; "dug" },
        0x1C => { memory[p[dst_pointer] as usize] = 0; p[dst_pointer] += 1; memory[p[dst_pointer] as usize] = 0; p[dst_pointer] += 1; "drg" },
        0x1D => { let temp = memory[p[dst_pointer] as usize]; memory[p[dst_pointer] as usize] = memory[p[dst_pointer] as usize + 1]; memory[p[dst_pointer] as usize + 1] = temp; "swp" },

        0x20 => { let temp = memory[p[src_pointer] as usize] + memory[p[src_pointer] as usize + 1]; memory[p[src_pointer] as usize] = 0; p[src_pointer] += 1; memory[p[src_pointer] as usize] = 0; p[src_pointer] += 1; p[dst_pointer] -= 1; memory[p[dst_pointer] as usize] = temp; "add" },
        //0x21 => not implemented, "adc"
        0x22 => { let temp = memory[p[src_pointer] as usize] - memory[p[src_pointer] as usize + 1]; memory[p[src_pointer] as usize] = 0; p[src_pointer] += 1; memory[p[src_pointer] as usize] = 0; p[src_pointer] += 1; p[dst_pointer] -= 1; memory[p[dst_pointer] as usize] = temp; "sub" },
        //0x23 => not implemented, "sbc"
        0x24 => { memory[p[dst_pointer] as usize] += 1; "inc" },
        0x25 => { memory[p[dst_pointer] as usize] -= 1; "dec" },
        // 0x26 => { let temp = if memory[*src_pointer as usize] > 127 { const_true } else { const_false }; *src_pointer += 1; *dst_pointer -= 1; memory[*dst_pointer as usize] = temp }, "ltz"
        // 0x27 => { let temp = if memory[*src_pointer as usize] < 127 { const_true } else { const_false }; *src_pointer += 1; *dst_pointer -= 1; memory[*dst_pointer as usize] = temp }, "gtz"
        0x28 => { let temp = if memory[p[src_pointer] as usize] != 0 { const_true } else { const_false }; p[src_pointer] += 1; p[dst_pointer] -= 1; memory[p[dst_pointer] as usize] = temp; "nez" },
        0x29 => { let temp = if memory[p[src_pointer] as usize] == 0 { const_true } else { const_false }; p[src_pointer] += 1; p[dst_pointer] -= 1; memory[p[dst_pointer] as usize] = temp; "eqz" },

        0x30 => { let temp = !memory[p[src_pointer] as usize]; memory[p[src_pointer] as usize] = 0; p[src_pointer] += 1; p[dst_pointer] -= 1; memory[p[dst_pointer] as usize] = temp; "not" },
        0x31 => { let temp = memory[p[src_pointer] as usize] | memory[p[src_pointer] as usize + 1]; memory[p[src_pointer] as usize] = 0; p[src_pointer] += 1; memory[p[src_pointer] as usize] = 0; p[src_pointer] += 1; p[dst_pointer] -= 1; memory[p[dst_pointer] as usize] = temp; "oor" },
        0x32 => { let temp = memory[p[src_pointer] as usize] & memory[p[src_pointer] as usize + 1]; memory[p[src_pointer] as usize] = 0; p[src_pointer] += 1; memory[p[src_pointer] as usize] = 0; p[src_pointer] += 1; p[dst_pointer] -= 1; memory[p[dst_pointer] as usize] = temp; "and" },
        0x33 => { let temp = memory[p[src_pointer] as usize] ^ memory[p[src_pointer] as usize + 1]; memory[p[src_pointer] as usize] = 0; p[src_pointer] += 1; memory[p[src_pointer] as usize] = 0; p[src_pointer] += 1; p[dst_pointer] -= 1; memory[p[dst_pointer] as usize] = temp; "xor" },
        0x34 => { memory[p[dst_pointer] as usize] = memory[p[dst_pointer] as usize] << 1; "shl" },
        0x35 => { memory[p[dst_pointer] as usize] = memory[p[dst_pointer] as usize] >> 1; "shr" },
        //0x36 => not implemented, "slc"
        //0x37 => not implemented, "src"
        //0x38 => not implemented, "sla"
        //0x39 => not implemented, "sra"

        _ => { println!("Invalid Instruction {:x?}", in_byte); break_type = const_unk; "unk" },
      };
    }
    if const_delay != 0 {
      println!("pointers: [stack, work, instruction]: {:02x} {:02x} {:02x}", p[const_stack], p[const_work], p[const_instruction]);
      if load_next {
        println!("value: {:x}", op_code);
      } else {
        // https://stackoverflow.com/questions/44690439/how-do-i-print-an-integer-in-binary-with-leading-zeros
        println!("op_code: {:02x}, prefix: {:02b}, mnemonic: {}{}", op_code, prefix, mnemonic, ["", "-s-w", "-w-s", "-w-w"][prefix as usize]);
      }
      // https://stackoverflow.com/questions/44549759/return-last-n-elements-of-vector-in-rust-without-mutating-the-vector
      println!("memory slice: {:02x?}", memory.as_slice()[memory.len()-0x16..].to_vec());
      println!("");
    }
    p[const_instruction] += 1;
    if break_type > 0 { break; }
    // https://stackoverflow.com/questions/28952938/how-can-i-put-the-current-thread-to-sleep
    thread::sleep(Duration::from_millis(const_delay));
  }
  println!("");
  // https://newbedev.com/get-last-element-of-vector-rust-code-example
  println!("Exit code: 0x{:x} ({})", memory.last().unwrap(), memory.last().unwrap());
  println!("CPU Halted. {}", const_break_lookup[break_type]);
}

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
