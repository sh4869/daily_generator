extern crate chrono;
extern crate clap;
extern crate dgen;

use chrono::{Datelike, Local, NaiveDate};
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
    let now = Local::now();
    let today = NaiveDate::from_ymd_opt(now.year(), now.month(), now.day());
    if matches.subcommand_matches("ytd").is_some() {
        let ytd = today.map_or(None, |r| r.pred_opt());
        match create_diary_template(ytd) {
            Ok(true) => println!("Create diary/{}.md", ytd.map_or("".to_string(), |r| r.format("%Y/%m/%d").to_string())),
            Ok(false) => println!("Error: diary/{}.md already exists.",ytd.map_or("".to_string(), |r| r.format("%Y/%m/%d").to_string())),
            Err(e) => println!("Error: {}", e.to_string()),
        }
    } else if matches.subcommand_matches("today").is_some() {
        match create_diary_template(today) {
            Ok(true) => println!("Create diary/{}.md", today.map_or("".to_string(), |r| r.format("%Y/%m/%d").to_string())),
            Ok(false) => println!("Error: diary/{}.md already exists.", today.map_or("".to_string(), |r| r.format("%Y/%m/%d").to_string())),
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
