use crate::note::Note;
use std::{process::Command, os::unix::process::CommandExt};

pub fn run_list() {
  Note::ensure_db_table();
  let notes = Note::all_notes().expect("Failed to get notes");
  let out = notes.into_iter().map(|note| note.title).collect::<Vec<String>>().join("\n");
  let mut cmd = Command::new("echo");
  cmd.arg(out);
  cmd.exec();
}