#![feature(proc_macro_hygiene)]
extern crate chrono;
extern crate fs_extra;
extern crate indicatif;
extern crate maud;
extern crate rss;
extern crate serde;
extern crate serde_json;
extern crate voca_rs;
extern crate pulldown_cmark;

pub mod diary;

use crate::diary::builders::*;
use crate::diary::{builder::BuildOption, builder::DiaryBuilder, builder::DiaryBuilderGen, parser::parse_daily};
use chrono::{Date, Local};
use std::fs::{self, File};
use std::io;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

pub fn build(dest: &str) -> io::Result<()> {
    let paths: Vec<PathBuf>;
    match glob::glob("diary/**/*.md") {
        Ok(v) => paths = v.flat_map(|x| x).collect::<Vec<_>>(),
        Err(e) => return Err(Error::new(ErrorKind::InvalidData, e.to_string())),
    }
    println!("|> parse diary source files");
    let mut v = Vec::new();
    let pb = indicatif::ProgressBar::new(paths.len() as u64);
    for path in paths {
        match parse_daily(path.as_path()) {
            Ok(daily) => {
                v.push(daily);
                pb.inc(1);
            }
            Err(e) => println!("\r\n{}", e),
        }
    }
    pb.finish_and_clear();
    let bp: BuildOption = BuildOption {
        dest: &dest,
        url: "https://diary.sh4869.sh",
    };
    let vec: Vec<Box<dyn DiaryBuilder>> = vec![
        Box::new(static_file_builder::StaticFileBuilder::new(&bp)),
        Box::new(diary_page_builder::DiaryPageBuilder::new(&bp)),
        Box::new(top_page_builder::TopPageBuilder::new(&bp)),
        Box::new(index_builder::IndexBuilder::new(&bp)),
        Box::new(rss_builder::RssBuilder::new(&bp)),
    ];
    for builder in vec {
        println!("|> build by {}", builder.builder_name());
        match builder.build(&mut v) {
            Ok(()) => (),
            Err(e) => println!("Error {}", e.to_string()),
        }
    }
    Ok(())
}

pub fn create_diary_template(date: Date<Local>) -> io::Result<bool> {
    let path = "diary/".to_string() + &date.format("%Y/%m/%d").to_string() + &".md";
    if !Path::new(&path).exists() {
        let parent = Path::new(&path).parent().unwrap();
        if parent.exists() == false {
            fs::create_dir_all(parent.to_str().unwrap())?;
        }
        let mut file = File::create(&path)?;
        file.write_all("---\ntitle:\n---\n".as_bytes())?;
        Ok(true)
    } else {
        Ok(false)
    }
}

