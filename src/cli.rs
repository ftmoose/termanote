use crate::note::run_note;
use crate::commands::{ScratchPadCommand, CliCommand, OpenNoteCommand, SearchNoteCommand};

fn run_help() {
    println!("termanote\n");
    println!("USAGE:");
    println!("tn [command|note title]\n");
    println!("COMMANDS:");
}

pub fn parse() {
    let command: String = std::env::args().nth(1).unwrap_or(String::new());
    let tail = match std::env::args().collect::<Vec<String>>().get(2..) {
        Some(a) => a.to_vec(),
        None => vec![]
    };
    
    match command.as_str() {
        ScratchPadCommand::COMMAND => ScratchPadCommand::run(&tail),
        OpenNoteCommand::COMMAND => OpenNoteCommand::run(&tail),
        SearchNoteCommand::COMMAND => SearchNoteCommand::run(&tail),
        // "list" => run_list(),
        // "rm" => {
        //     let note_title: String = std::env::args().collect::<Vec<String>>()[2..].join(" ");
        //     println!("{}", note_title);
        //     run_rm(&note_title)
        // },
        "help" => run_help(),
        _ => {
            let note_title: String = std::env::args().collect::<Vec<String>>()[1..].join(" ");
            run_note(&note_title)
        }
    }
}