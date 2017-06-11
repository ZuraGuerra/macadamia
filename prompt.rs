use std::io;
use std::io::prelude::*;

fn print_info() {
  println!("*** MACADAMIA ***");
  println!("Version 0.0.0.0.1\n\n");
  println!("Press Ctrl+c to Exit");
}

fn output_prompt() {
  write!(&mut io::stdout(), "><}}*> ");
  io::stdout().flush();
}

fn print_input(input: String) {
  writeln!(&mut io::stdout(), "><}}*< {:?}", input);
}


fn main() {
  print_info();

  loop {
    output_prompt();

    let mut input = String::new();
    io::stdin().read_line(&mut input);

    if input == "exit\n" {
      break;
    }

    print_input(input);
  }
}
