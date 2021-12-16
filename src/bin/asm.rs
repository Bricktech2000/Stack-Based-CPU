use std::env;
use std::str;
use std::fs;
use std::collections::HashMap;

fn main() {
  let const_arg = 0;
  let const_eof = 1;
  let const_lbl = 2;
  let const_ok = 3;
  let const_break_lookup: [&str; 4] = [
    "Error.",
    "Error: Unexpected EOF.",
    "Error: Label not found.",
    "Success.",
  ];

  let args: Vec<String> = env::args().collect();
  if args.len() != 2 {
    println!("Usage: asm <filename>");
    return;
  }

  println!("Assembling Binary...");

  let in_string: String = fs::read_to_string(&args[1]).expect("Unable to read file.");
  let in_bytes: &[u8] = in_string.as_bytes();
  let mut mod_string: String = String::new();
  let mut out_bytes: Vec<u8> = vec![];
  // let mut in_tokens: Vec<&str> = in_string.split_whitespace().collect();

  let mut last_was_offset: bool = false; // TODO
  let mut label_to_value: HashMap<String, u8> = HashMap::new();
  let mut mention_to_label: HashMap<u8, String> = HashMap::new();

  let mut index = 0;
  while index <  in_bytes.len() {
    if in_bytes[index] as char == '#' {
      while in_bytes[index] as char != '\n' { index += 1; }
      index += 1;
      continue;
    }
    mod_string.push(in_bytes[index] as char);
    index += 1;
  }
  let tokens: Vec<String> = mod_string.split_whitespace().map(str::to_string).collect();
  println!("{:?}", tokens);

  // while index < in_bytes.len() {
  //   let in_char: u8 = in_bytes[index];
  //   // ignore whitespace and comments
  //   if " \n\t".contains(in_char as char) {
  //     index += 1;
  //     continue;
  //   }
  //   current_tok.push(in_char as char);
  //   if last_was_offset && current_tok.as_str() != "x" {
  //     println!("Error: Expected argument.");
  //     break_type = const_arg;
  //     break;
  //   }

  //   let matched: bool = match current_tok.as_str() {
  //     "x" | "p" => {
  //       let mut hex_str: String = String::new();
  //       index += 1;
  //       hex_str.push(in_bytes[index] as char);
  //       index += 1;
  //       hex_str.push(in_bytes[index] as char);
  //       let hex_num = u8::from_str_radix(hex_str.as_str(), 16).expect("Unable to parse hex.");
  //       if current_tok.as_str() == "x" {
  //         if last_was_offset {
  //           last_was_offset = false;
  //           if hex_num < 0x40 {
  //             let instruction = out_bytes.pop().unwrap();
  //             out_bytes.push(instruction | hex_num);
  //           } else {
  //             println!("Error: Invalid argument {} for instruction.", hex_num);
  //             break_type = const_arg;
  //           }
  //         } else if hex_num < 0x10 {
  //           out_bytes.push(0xF0 | hex_num);
  //         } else {
  //           out_bytes.push(0x01);
  //           out_bytes.push(hex_num);
  //         }
  //       } else {
  //         out_bytes.push(hex_num);
  //       }
  //       true },
  //     "nop" => {
  //       out_bytes.push(0x00);
  //       true },
  //     "hlt" => {
  //       out_bytes.push(0x02);
  //       true },
  //     "out" => {
  //       out_bytes.push(0x08);
  //       true },
  //     "iin" => {
  //       out_bytes.push(0x09);
  //       true },

  //     "lda" => {
  //       out_bytes.push(0x11);
  //       true },
  //     "sta" => {
  //       out_bytes.push(0x12);
  //       true },
  //     "lds" => {
  //       out_bytes.push(0x13);
  //       true },
  //     "sts" => {
  //       out_bytes.push(0x14);
  //       true },
  //     "ldi" => {
  //       out_bytes.push(0x15);
  //       true },
  //     "sti" => {
  //       out_bytes.push(0x16);
  //       true },
  //     "ldp" => {
  //       out_bytes.push(0x17);
  //       true },
  //     "stp" => {
  //       out_bytes.push(0x18);
  //       true },

  //     "dup" => {
  //       out_bytes.push(0x19);
  //       true },
  //     "drp" => {
  //       out_bytes.push(0x1A);
  //       true },
  //     "swp" => {
  //       out_bytes.push(0x1B);
  //       true },

  //     "add" => {
  //       out_bytes.push(0x20);
  //       true },
  //     "adc" => {
  //       out_bytes.push(0x21);
  //       true },
  //     "sub" => {
  //       out_bytes.push(0x22);
  //       true },
  //     "sbc" => {
  //       out_bytes.push(0x23);
  //       true },
  //     "inc" => {
  //       out_bytes.push(0x24);
  //       true },
  //     "dec" => {
  //       out_bytes.push(0x25);
  //       true },
  //     "ilt" => {
  //       out_bytes.push(0x26);
  //       true },
  //     "lgt" => {
  //       out_bytes.push(0x27);
  //       true },
  //     "ieq" => {
  //       out_bytes.push(0x28);
  //       true },
  //     "nez" => {
  //       out_bytes.push(0x29);
  //       true },
  //     "neg" => {
  //       out_bytes.push(0x2A);
  //       true },
  //     "abs" => {
  //       out_bytes.push(0x2B);
  //       true },
      
  //     "not" => {
  //       out_bytes.push(0x30);
  //       true },
  //     "oor" => {
  //       out_bytes.push(0x31);
  //       true },
  //     "and" => {
  //       out_bytes.push(0x32);
  //       true },
  //     "xor" => {
  //       out_bytes.push(0x33);
  //       true },
  //     "xnd" => {
  //       out_bytes.push(0x34);
  //       true },

  //     "sto" => {
  //       last_was_offset = true;
  //       out_bytes.push(0x40);
  //       true },
  //     "ldo" => {
  //       last_was_offset = true;
  //       out_bytes.push(0x80);
  //       true },
  //     "skp" => {
  //       // TODO: could overflow, see "x" | "p"
  //       last_was_offset = true;
  //       out_bytes.push(0xC0);
  //       true },
  //     //TODO: shifts

  //     "#" => {
  //       index += 1;
  //       while in_bytes[index] as char != '\n' { index += 1; }
  //       true },
  //     "$" => {
  //       let mut label: String = String::new();
  //       while !" \n\t".contains(in_bytes[index + 1] as char) {
  //         index += 1;
  //         label.push(in_bytes[index] as char);
  //       }
  //       out_bytes.push(0x01);
  //       out_bytes.push(0xCC);
  //       mention_to_label.insert(out_bytes.len() as u8 - 1, label);
  //       true },
  //     "lbl$" => {
  //       let mut label: String = String::new();
  //       while !" \n\t".contains(in_bytes[index + 1] as char) {
  //         index += 1;
  //         label.push(in_bytes[index] as char);
  //       }
  //       label_to_value.insert(label, out_bytes.len() as u8);
  //       true },
  //     _ => { false },
  //   };

  //   if matched { current_tok = String::new(); };

  //   index += 1;
  //   if break_type != const_ok { break; }
  // }
  // for (mention, label) in mention_to_label.iter() {
  //   match label_to_value.get(label) {
  //     Some(value) => {
  //       out_bytes[*mention as usize] = *value
  //     },
  //     None => { break_type = const_lbl; println!("Label {} not found.", label) }
  //   }
  // }

  // if break_type == const_ok && current_tok.len() > 0 { break_type = const_eof; }

  // println!("Assembly complete. {}", const_break_lookup[break_type]);
  // if break_type == const_ok {
  //   println!("Writing bytes...");
  //   fs::write(format!("{}{}", &args[1], ".bin"), out_bytes).expect("Unable to write file.");
  //   println!("Done.");
  // }
}

fn die(code: usize, instruction_pointer: u8, value: u8) {
  let message: &str = [
    "Success ",
    "",
  ][code];

  println!("Fatal Error at {:02x}: {}{:02x}.", instruction_pointer, message, value);
  println!("Exiting.");
  std::process::exit(code as i32);
}
