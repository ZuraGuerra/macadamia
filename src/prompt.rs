use std::io;
use std::io::prelude::*;
extern crate rustyline;

pub fn print_info() {
  println!("*** MACADAMIA ***");
  println!("Version 0.0.1\n\n");
  println!("Press Ctrl+c to Exit");
}

pub fn repl() {
  let mut prompt = rustyline::Editor::<()>::new();
  loop {
    let input = prompt.readline("><}}*> ");

    match input {
      Ok(line) => {
        prompt.add_history_entry(line.as_str());
        print_input(line);
      },
      Err(_) => break,
    }
  }
}

fn print_input(input: String) {
  writeln!(&mut io::stdout(), "><}}*< {:?}", input);
}
