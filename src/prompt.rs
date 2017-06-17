use std::io;
use std::io::prelude::*;
extern crate rustyline;

pub fn print_info() {
  println!("*** MACADAMIA ***");
  println!("Version 0.0.1\n\n");
  println!("Press Ctrl+c to Exit");
}

pub fn output_prompt() {
  write!(&mut io::stdout(), "><}}*> ");
  io::stdout().flush();
}

pub fn print_input(input: String) {
  writeln!(&mut io::stdout(), "><}}*< {:?}", input);
}
