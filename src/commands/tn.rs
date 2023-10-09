use crate::commands::{CliCommand, OpenNoteCommand};

pub struct ScratchPadCommand {}

impl ScratchPadCommand {
  pub const COMMAND: &str = "";
}

impl CliCommand for ScratchPadCommand {
  fn run(_: &Vec<String>) -> () {
    OpenNoteCommand::run(&vec![String::from("scratchpad")]);
  }
}