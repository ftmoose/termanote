use std::io::Write;
use crate::note::Note;

pub fn run_rm(title: &String) {
  if title.trim().is_empty() {
    println!("No note given.");
    return
  }

  let mut note = match Note::find_by_title(title) {
    Some(note) => note,
    None => {
      println!("No note named {} found.", title);
      return
    }
  };

  println!("Removing note: {}", note.title);
  print!("This can't be undone, continue? (y/N): ");
  std::io::stdout().flush().expect("Failed to flush stdout");
  let mut continue_input = String::new();
  std::io::stdin()
    .read_line(&mut continue_input)
    .expect("Failed to read line");

  match continue_input {
    ref mut s if s.trim().to_lowercase() == "y" => {
      // delete note
      note.delete();
      println!("Deleted {}", note.title)
    },
    _ => return,
  }
}