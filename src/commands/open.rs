use std::fs;
use std::process::Command;
use crate::note::Note;
use crate::commands::CliCommand;

pub struct OpenNoteCommand {}

impl OpenNoteCommand {
  pub const COMMAND: &str = "open";
}

impl CliCommand for OpenNoteCommand {
  fn run(args: &Vec<String>) -> () {
    // make sure we have a title
    if args.len() == 0 {
      panic!("New note needs a title!");
    }

    // get title
    let title = args.join(" ");

    // setup note
    Note::ensure_db_table();
    let mut note = Note::find_by_title(&title, true);
    
    // open note
    let tmp_dir = "./.tmp";
    let _ = fs::create_dir(tmp_dir);
    let tmp_file_path = format!("{}/.{}.tmp.tn", tmp_dir, note.id);
    fs::write(
      tmp_file_path.clone(),
      note.content.unwrap_or(vec![])
    ).expect("Cannot open note! Is it already open?");

    let mut cmd = Command::new("vim");
    cmd.arg(tmp_file_path.clone());
    let mut child = cmd.spawn().expect("Failed to spawn vim, do you have it installed?");

    let status = child.wait().expect("Failed to wait on vim, what happened!?");
    if status.success() {
      // write the contents of the file to the note
      let content = fs::read(tmp_file_path.clone()).expect("Failed to read tmp file");
      note.content = Some(content);
      note.save();

      // delete tmp file
      fs::remove_file(tmp_file_path.clone()).expect("Failed to remove tmp file");
    } else {
      panic!("Error while editing note, what happened!?")
    } 
  }
}