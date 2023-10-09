use super::CliCommand;

pub struct DeleteNoteCommand {}

impl DeleteNoteCommand {
  pub const COMMAND: &str = "open";
}

impl CliCommand for DeleteNoteCommand {
  fn run(args: &Vec<String>) -> () {
    print!("Delete Note!")
  }
}