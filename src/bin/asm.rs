use std::env;
use std::fs;

fn main() {
  // https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
  let args: Vec<String> = env::args().collect();
  if args.len() != 2 {
    println!("Usage: ./asm <filename>");
    return;
  }

  println!("Assembling Hex...");

  // https://stackoverflow.com/questions/31192956/whats-the-de-facto-way-of-reading-and-writing-files-in-rust-1-x
  let hex_bytes = fs::read_to_string(&args[1]).expect("Unable to read file.");

  let mut out_bytes: Vec<u8> = vec![];
  let mut accumulator: u8 = 0;
  let mut first_byte = true;
  // println!("{:?}", get_file_as_byte_vec(&args[1]));
  // https://doc.rust-lang.org/rust-by-example/flow_control/for.html
  // https://doc.rust-lang.org/rust-by-example/types/cast.html
  let hex_chars: Vec<char> = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];
  for hex_char in hex_bytes.chars() {
    // https://blog.mgattozzi.dev/how-do-i-str-string/
    if " \n\t".contains(hex_char) {
      // ignore whitespace
      continue;
    }
    if hex_chars.contains(&hex_char) {
      // https://stackoverflow.com/questions/30558246/how-do-i-find-the-index-of-an-element-in-an-array-vector-or-slice
      let digit = hex_chars.iter().position(|&r| r == hex_char).unwrap() as u8;
      accumulator <<= 4;
      accumulator |= digit;
      first_byte = !first_byte;

      if first_byte {
        out_bytes.push(accumulator);
        accumulator = 0;
      }
    } else {
      // probably a comment
      continue;
    }
    // println!("{}", *hex_byte as char);
  }
  println!("Writing bytes: {:x?}", out_bytes);
  fs::write(format!("{}{}", &args[1], ".bin"), out_bytes).expect("Unable to write file.");
}
