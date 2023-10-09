use crate::db;
use rusqlite::params;
use std::fs;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct Note {
  pub id: u32,
  pub title: String,
  pub content: Option<Vec<u8>>,
}

impl Note {
  pub fn new(title: &String) -> Self {
    println!("creating new note!");
    let conn = db::connect();
    conn.execute(
      "INSERT INTO note (title) VALUES (?1)",
      (title,),
    ).expect("Failed to insert note");

    let mut stmt = conn.prepare("SELECT id, title, content FROM note WHERE title = ?1")
      .expect("Failed to prepare note query");
    let note_iter = stmt.query_map(params![title], |row| {
      Ok(Note {
        id: row.get(0)?,
        title: row.get(1)?,
        content: row.get(2)?,
      })
    }).unwrap().collect::<Result<Vec<Note>, _>>().expect("Failed to collect note");

    note_iter.first().unwrap().clone()
  }

  pub fn ensure_db_table() {
    let conn = db::connect();
    let mut statement = conn.prepare(
    r#"
      CREATE TABLE IF NOT EXISTS note (
        id INTEGER PRIMARY KEY,
        title TEXT NOT NULL UNIQUE,
        content BLOB
      )
    "#
    ).expect("Failed to prepare statement");

    statement.execute(params![]).expect("Failed to create table");
  }

  pub fn find_by_title(title: &String, create: bool) -> Note {
    let conn = db::connect();
    let mut statement = conn.prepare(
      "SELECT id, title, content FROM note WHERE title = ?1"
    ).expect("Failed to prepare statement");

    let notes = statement.query_map(params![title], |row| {
      Ok(Note {
        id: row.get(0)?,
        title: row.get(1)?,
        content: row.get(2)?,
      })
    }).unwrap().collect::<Result<Vec<Note>, _>>().expect("Failed to collect note");

    let note = match notes.into_iter().nth(0) {
      Some(note) => Some(note),
      None => if create {
        Some(Note::new(title))
      } else {
        None
      },
    }; 

    if note.is_none() {
      panic!("Note '{}' does not exist", title)
    }

    note.unwrap()
  }

  pub fn all_notes() -> Result<Vec<Note>, rusqlite::Error> {
    let conn = db::connect();
    let mut statement = conn.prepare(
      "SELECT id, title, content FROM note"
    ).expect("Failed to prepare statement");

    let notes = statement.query_map([], |row| {
      Ok(Note {
        id: row.get(0)?,
        title: row.get(1)?,
        content: row.get(2)?,
      })
    }).unwrap().collect::<Result<Vec<Note>, _>>();

    notes
  }

  pub fn save(&mut self) {
    let conn = db::connect();
    conn.execute(
      "UPDATE note SET content = ?1 WHERE id = ?2",
      (&self.content, &self.id),
    ).expect("Failed to update note");
  }

  pub fn delete(&mut self) {
    let conn = db::connect();
    conn.execute(
      "DELETE FROM note WHERE id = ?1",
      (&self.id,),
    ).expect("Failed to delete note");
  }
}

pub fn run_note(title: &String) {
  Note::ensure_db_table();

  // find or create note
  let title = &title.clone().trim().to_lowercase();
  let mut note = Note::find_by_title(title, false);

  println!("New note id: {}", note.id);

  // create tmp file for editing note
  let tmp_dir = "./.tmp";
  let _ = fs::create_dir(tmp_dir);
  let tmp_file_path = format!("{}/.{}.tmp.tn", tmp_dir, note.id);
  fs::write(tmp_file_path.clone(), note.content.unwrap_or(vec![]))
    .expect("Failed to write tmp file");
  
  // open tmp file in editor
  let mut cmd = Command::new("vim");
  cmd.arg(tmp_file_path.clone());
  let mut child = cmd.spawn().expect("Failed to spawn vim");

  // wait for the user to finish editing
  let status = child.wait().expect("Failed to wait on vim");
  if status.success() {
    // write the contents of the file to the note
    let content = fs::read(tmp_file_path.clone()).expect("Failed to read tmp file");
    note.content = Some(content);
    note.save();

    // delete tmp file
    fs::remove_file(tmp_file_path.clone()).expect("Failed to remove tmp file");
  } else {
    println!("Error while editing note!");
  }
}