extern crate chrono;
extern crate clap;
extern crate diary_generator;

use chrono::Local;
use clap::{App, Arg, SubCommand};

use diary_generator::{build, create_diary_template, create_templates};

fn main() {
    let matches = App::new("Daily Generator")
        .version("0.1")
        .author("sh4869 <nobuk4869@gmail.com>")
        .about("generate daily program")
        .subcommand(
            SubCommand::with_name("new")
                .about("generate new file")
                .arg(Arg::with_name("all").short("a").help("generate all diary not created")),
        )
        .get_matches();

    if let Some(matches_new) = matches.subcommand_matches("new") {
        if matches_new.is_present("all") {
            match create_templates(Local::today() - chrono::Duration::days(15)) {
                Ok(()) => println!(">>> Created templates"),
                Err(e) => println!("Error: {}", e.to_string()),
            }
        } else {
            match create_diary_template(Local::today().pred()) {
                Ok(true) => println!(">>> Create diary/{}.md", Local::today().format("%Y/%m/%d")),
                Ok(false) => {}
                Err(e) => println!("Error: {}", e.to_string()),
            }
        }
    } else {
        println!("> Build Diary...");
        match build() {
            Ok(()) => println!("> All Dailies build completed."),
            Err(e) => println!("Error: {}", e.to_string()),
        }
    }
}
