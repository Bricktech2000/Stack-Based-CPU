use std::env;
use std::str;
use std::fs;

fn main() {
  let const_arg = 0;
  let const_eof = 1;
  let const_ok = 2;
  let const_break_lookup: [&str; 3] = [
    "Error.",
    "Error: Unexpected EOF.",
    "Success.",
  ];

  // https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
  let args: Vec<String> = env::args().collect();
  if args.len() != 2 {
    println!("Usage: asm <filename>");
    return;
  }

  println!("Assembling Binary...");

  // https://stackoverflow.com/questions/31192956/whats-the-de-facto-way-of-reading-and-writing-files-in-rust-1-x
  // https://stackoverflow.com/questions/23975391/how-to-convert-a-string-into-a-static-str
  let in_bytes: Vec<u8> = fs::read(&args[1]).expect("Unable to read file.");
  let mut out_bytes: Vec<u8> = vec![];
  let mut current_tok: String = String::new();
  let mut index: usize = 0;
  let mut last_was_offset: bool = false;

  let mut break_type = const_ok;
  while index < in_bytes.len() {
    let in_char: u8 = in_bytes[index];
    // https://blog.mgattozzi.dev/how-do-i-str-string/
    // ignore whitespace and comments
    if " \n\t".contains(in_char as char) {
      index += 1;
      continue;
    }
    current_tok.push(in_char as char);
    if last_was_offset && current_tok.as_str() != "x" {
      println!("Error: Expected argument.");
      break_type = const_arg;
      break;
    }

    // https://stackoverflow.com/questions/19076719/how-do-i-convert-a-vector-of-bytes-u8-to-a-string
    // https://stackoverflow.com/questions/25383488/how-to-match-a-string-against-string-literals
    let matched: bool = match current_tok.as_str() {
      "x" | "p" => {
        // github copilot magic
        let mut hex_str: String = String::new();
        index += 1;
        hex_str.push(in_bytes[index] as char);
        index += 1;
        hex_str.push(in_bytes[index] as char);
        // https://stackoverflow.com/questions/32381414/converting-a-hexadecimal-string-to-a-decimal-integer
        let hex_num = u8::from_str_radix(hex_str.as_str(), 16).expect("Unable to parse hex.");
        if current_tok.as_str() == "x" {
          if last_was_offset {
            last_was_offset = false;
            if hex_num < 0x40 {
              let instruction = out_bytes.pop().unwrap();
              out_bytes.push(instruction | hex_num);
            } else {
              println!("Error: Invalid argument {} for instruction.", hex_num);
              break_type = const_arg;
            }
          } else if hex_num < 0x10 {
            out_bytes.push(0xF0 | hex_num);
          } else {
            out_bytes.push(0x01);
            out_bytes.push(hex_num);
          }
        } else {
          out_bytes.push(hex_num);
        }
        true },
      "nop" => {
        out_bytes.push(0x00);
        true },
      "hlt" => {
        out_bytes.push(0x02);
        true },
      "out" => {
        out_bytes.push(0x08);
        true },
      "iin" => {
        out_bytes.push(0x09);
        true },

      "lda" => {
        out_bytes.push(0x11);
        true },
      "sta" => {
        out_bytes.push(0x12);
        true },
      "lds" => {
        out_bytes.push(0x13);
        true },
      "sts" => {
        out_bytes.push(0x14);
        true },
      "ldi" => {
        out_bytes.push(0x15);
        true },
      "sti" => {
        out_bytes.push(0x16);
        true },
      "ldp" => {
        out_bytes.push(0x17);
        true },
      "stp" => {
        out_bytes.push(0x18);
        true },

      "dup" => {
        out_bytes.push(0x19);
        true },
      "drp" => {
        out_bytes.push(0x1A);
        true },
      "swp" => {
        out_bytes.push(0x1B);
        true },

      "add" => {
        out_bytes.push(0x20);
        true },
      "adc" => {
        out_bytes.push(0x21);
        true },
      "sub" => {
        out_bytes.push(0x22);
        true },
      "sbc" => {
        out_bytes.push(0x23);
        true },
      "inc" => {
        out_bytes.push(0x24);
        true },
      "dec" => {
        out_bytes.push(0x25);
        true },
      "ilt" => {
        out_bytes.push(0x26);
        true },
      "lgt" => {
        out_bytes.push(0x27);
        true },
      "ieq" => {
        out_bytes.push(0x28);
        true },
      "nez" => {
        out_bytes.push(0x29);
        true },
      "neg" => {
        out_bytes.push(0x2A);
        true },
      "abs" => {
        out_bytes.push(0x2B);
        true },
      
      "not" => {
        out_bytes.push(0x30);
        true },
      "oor" => {
        out_bytes.push(0x31);
        true },
      "and" => {
        out_bytes.push(0x32);
        true },
      "xor" => {
        out_bytes.push(0x33);
        true },
      "xnd" => {
        out_bytes.push(0x34);
        true },

      "sto" => {
        last_was_offset = true;
        out_bytes.push(0x40);
        true },
      "ldo" => {
        last_was_offset = true;
        out_bytes.push(0x80);
        true },
      //shifts

      "#" => {
        index += 1;
        while in_bytes[index] as char != '\n' { index += 1; }
        true },
      _ => { false },
    };

    if matched { current_tok = String::new(); };

    index += 1;
    if break_type != const_ok { break; }
  }
  if break_type == const_ok && current_tok.len() > 0 { break_type = const_eof; }

  println!("Assembly complete. {}", const_break_lookup[break_type]);
  // https://doc.rust-lang.org/rust-by-example/flow_control/for.html
  // https://doc.rust-lang.org/rust-by-example/types/cast.html
  if break_type == const_ok {
    println!("Writing bytes...");
    fs::write(format!("{}{}", &args[1], ".bin"), out_bytes).expect("Unable to write file.");
    println!("Done.");
  }
}
