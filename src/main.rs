extern crate chrono;
extern crate clap;
extern crate dgen;

use chrono::Local;
use clap::{App, Arg, SubCommand};

use dgen::{build, create_diary_template};

fn main() {
    let matches = App::new("diary generator")
        .version("0.1.1")
        .author("sh4869 <nobuk4869@gmail.com>")
        .about("diary generator")
        .subcommand(SubCommand::with_name("ytd").about("generate diary template file of yesterday"))
        .subcommand(SubCommand::with_name("today").about("generate diary template file of yesterday"))
        .arg(Arg::with_name("dest").short("d").long("dest").help("dest directory of generated dailies file").takes_value(true))
        .get_matches();

    if matches.subcommand_matches("ytd").is_some() {
        match create_diary_template(Local::today().pred()) {
            Ok(true) => println!("Create diary/{}.md", Local::today().pred().format("%Y/%m/%d")),
            Ok(false) => println!("Error: diary/{}.md already exists.", Local::today().pred().format("%Y/%m/%d")),
            Err(e) => println!("Error: {}", e.to_string()),
        }
    } else if matches.subcommand_matches("today").is_some() {
        match create_diary_template(Local::today()) {
            Ok(true) => println!("Create diary/{}.md", Local::today().format("%Y/%m/%d")),
            Ok(false) => println!("Error: diary/{}.md already exists.", Local::today().format("%Y/%m/%d")),
            Err(e) => println!("Error: {}", e.to_string()),
        }
    } else {
        println!("building diary...");
        let dest = matches.value_of("dest").unwrap_or("docs");
        match build(dest) {
            Ok(()) => println!("complete!"),
            Err(e) => println!("Error: {}", e.to_string()),
        }
    }
}
