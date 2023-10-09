use crate::commands::CliCommand;

pub struct SearchNoteCommand {}

impl SearchNoteCommand {
  pub const COMMAND: &str = "search";
}

impl CliCommand for SearchNoteCommand {
  fn run(_: &Vec<String>) -> () {

  }
}