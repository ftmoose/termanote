mod cli;
mod note;
mod db;
mod commands;

fn main() {
    note::Note::ensure_db_table();
    cli::parse();
}