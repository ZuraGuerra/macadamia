extern crate macadamia;
use std::io;

fn main() {
  macadamia::prompt::print_info();

  loop {
    macadamia::prompt::output_prompt();

    let mut input = String::new();
    io::stdin().read_line(&mut input);

    if input == "exit\n" {
      break;
    }

    macadamia::prompt::print_input(input);
  }
}
