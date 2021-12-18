use std::env;
use std::str;
use std::fs;
use std::collections::HashMap;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() != 2 {
    println!("Usage: asm <filename>");
    return;
  }

  println!("Running Assembler...");

  let in_string: String = fs::read_to_string(&args[1]).expect("Unable to read file.");
  let out_bytes: Vec<u8> = assemble(&in_string);
  fs::write(format!("{}{}", &args[1], ".bin"), out_bytes).expect("Unable to write file.");

  println!("Process Successful.");
}

fn assemble(in_string: &String) -> Vec<u8> {
  let in_bytes: &[u8] = in_string.as_bytes();

  // filter out comments
  let mut index = 0;
  let mut mod_string: String = String::new();
  while index <  in_bytes.len() {
    if in_bytes[index] as char == '#' {
      while in_bytes[index] as char != '\n' { index += 1; }
      index += 1;
      continue;
    }
    mod_string.push(in_bytes[index] as char);
    index += 1;
  }
  // split into individual tokens
  let tokens: Vec<&str> = mod_string.split_whitespace().collect();
  // used to resolve labels
  let mut label_to_value: HashMap<String, u8> = HashMap::new();
  let mut mention_to_label: HashMap<u8, String> = HashMap::new();
  let mut out_bytes = assemble_recursive(tokens, 0x00, &mut label_to_value, &mut mention_to_label);
  // resolve labels
  for (mention, label) in mention_to_label.iter() {
    match label_to_value.get(label) {
      Some(value) => out_bytes[*mention as usize] = *value,
      None => die(0x04, label),
    }
  }
  out_bytes
}

fn assemble_recursive(tokens: Vec<&str>, offset: usize, label_to_value: &mut HashMap<String, u8>, mention_to_label: &mut HashMap<u8, String>) -> Vec<u8> {
  let mut index = 0;
  let mut out_bytes: Vec<u8> = vec![];
  while index < tokens.len() {
    let current_token: &str = tokens[index];

    match current_token {
      "nop" => { out_bytes.push(0x00) },
      "hlt" => { out_bytes.push(0x02) },
      "dbg" => { out_bytes.push(0x0F) },
      "jms" => { index += 1; out_bytes.append(&mut assemble_recursive(vec!["ldi", "x05", "add", tokens[index], "sti"], out_bytes.len(), label_to_value, mention_to_label)) },
      "rts" => { out_bytes.append(&mut assemble_recursive(vec!["sti"], out_bytes.len(), label_to_value, mention_to_label)) },

      "lda" => { out_bytes.push(0x11) },
      "sta" => { out_bytes.push(0x12) },
      "lds" => { out_bytes.push(0x13) },
      "sts" => { out_bytes.push(0x14) },
      "ldi" => { out_bytes.push(0x15) },
      "sti" => { out_bytes.push(0x16) },
      "ldp" => { out_bytes.push(0x17) },
      "stp" => { out_bytes.push(0x18) },
      "ldb" => { out_bytes.push(0x19) },
      "stb" => { out_bytes.push(0x1A) },
      "dup" => { out_bytes.push(0x1B) },
      "drp" => { out_bytes.push(0x1C) },
      "swp" => { out_bytes.push(0x1D) },

      "add" => { out_bytes.push(0x20) },
      "adc" => { out_bytes.push(0x21) },
      "sub" => { out_bytes.push(0x22) },
      "sbc" => { out_bytes.push(0x23) },
      "inc" => { out_bytes.push(0x24) },
      "dec" => { out_bytes.push(0x25) },
      "ilt" => { out_bytes.push(0x26) },
      "lgt" => { out_bytes.push(0x27) },
      "ieq" => { out_bytes.push(0x28) },
      "nez" => { out_bytes.push(0x29) },
      "neg" => { out_bytes.push(0x2A) },
      "abs" => { out_bytes.push(0x2B) },

      "not" => { out_bytes.push(0x30) },
      "oor" => { out_bytes.push(0x31) },
      "and" => { out_bytes.push(0x32) },
      "xor" => { out_bytes.push(0x33) },
      "xnd" => { out_bytes.push(0x34) },

      "sto" | "ldo" | "skp" | "shl" | "shr" => {
        index += 1;
        let op_code = match current_token {
          "sto" => 0x40,
          "ldo" => 0x80,
          "skp" => 0xC0, // TODO: could overflow
          "shl" => 0xD0, // TODO: could overflow
          "shr" => 0xD8, // TODO: could overflow
          _ => { die(0x05, current_token); 0x00 },
        };
        let current_token = tokens[index];
        match get_immediate(current_token) {
          Ok(value) => {
            if value < 0x20 { out_bytes.push(op_code | value) }
            else { die(0x02, current_token) }
          },
          Err(code) => die(code, current_token),
        }
      },
      //TODO: shifts

      "lbl" => {
        index += 1;
        let current_token = tokens[index];
        label_to_value.insert(current_token.to_string(), (offset + out_bytes.len()) as u8);
      },
      _ if current_token.starts_with("$") => {
        out_bytes.push(0x01);
        out_bytes.push(0xCC);
        mention_to_label.insert((offset + out_bytes.len()) as u8 - 1, current_token.to_string());
      },
      _ if current_token.starts_with("x") => {
        match get_immediate(current_token) {
          Ok(value) => {
            if value < 0x10 {
              out_bytes.push(0xF0 | value);
            } else {
              out_bytes.push(0x01);
              out_bytes.push(value);
            }
          },
          Err(code) => die(code, current_token),
        }
      },
      _ if current_token.starts_with("p") => {
        match get_immediate(current_token) {
          Ok(value) => out_bytes.push(value),
          Err(code) => die(code, current_token),
        }
      },
      _ => { die(0x03, current_token) },
    };
    index += 1;
  }
  out_bytes
}

fn get_immediate(current_token: &str) -> Result<u8, usize> {
  let hex_str = &current_token[1..];
  let hex_num = u8::from_str_radix(hex_str, 16).expect("Unable to parse hex.");
  Ok(hex_num)
}

fn die(code: usize, value: &str) {
  let message: &str = [
    "Success ",
    "Invalid immediate: ",
    "Immediate Overflow: ",
    "Invalid Instruction: ",
    "Label Not Found: ",
    "Internal Error: Unknown opcode: ",
  ][code];

  println!("Fatal Error: {}{}.", message, value);
  println!("Exiting.");
  std::process::exit(code as i32);
}
