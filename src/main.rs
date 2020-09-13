extern crate chrono;
extern crate clap;
extern crate diary_generator;

use chrono::Local;
use clap::{App, SubCommand};

use diary_generator::{build, create_diary_template};

fn main() {
    let matches = App::new("Daily Generator")
        .version("0.1")
        .author("sh4869 <nobuk4869@gmail.com>")
        .about("generate daily program")
        .subcommand(SubCommand::with_name("ytd").about("generate new file"))
        .subcommand(SubCommand::with_name("today").about("generate today file"))
        .get_matches();

    if matches.subcommand_matches("ytd").is_some() {
        match create_diary_template(Local::today().pred()) {
            Ok(true) => println!(">>> Create diary/{}.md", Local::today().pred().format("%Y/%m/%d")),
            Ok(false) => println!("diary/{}.md already exists.", Local::today().pred().format("%Y/%m/%d")),
            Err(e) => println!("Error: {}", e.to_string()),
        }
    } else if matches.subcommand_matches("today").is_some() {
        match create_diary_template(Local::today()) {
            Ok(true) => println!(">>> Create diary/{}.md", Local::today().format("%Y/%m/%d")),
            Ok(false) => println!("diary/{}.md already exists.", Local::today().format("%Y/%m/%d")),
            Err(e) => println!("Error: {}", e.to_string()),
        }
    } else {
        println!("> Build Diary...");
        match build() {
            Ok(()) => println!("> All Dailies build completed."),
            Err(e) => println!("Error: {}", e.to_string()),
        }
    }
}
