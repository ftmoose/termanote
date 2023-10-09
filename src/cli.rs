use crate::note::run_note;
use crate::list::run_list;
use crate::rm::run_rm;

fn run_help() {
    println!("termanote\n");
    println!("USAGE:");
    println!("tn [command|note title]\n");
    println!("COMMANDS:");
}

pub fn parse() {
    let command: String = std::env::args().nth(1).unwrap_or("help".to_string());
    
    match command.as_str() {
        "list" => run_list(),
        "rm" => {
            let note_title: String = std::env::args().collect::<Vec<String>>()[2..].join(" ");
            println!("{}", note_title);
            run_rm(&note_title)
        },
        "help" => run_help(),
        _ => {
            let note_title: String = std::env::args().collect::<Vec<String>>()[1..].join(" ");
            run_note(&note_title)
        }
    }
}