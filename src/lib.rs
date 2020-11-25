#![feature(proc_macro_hygiene)]
extern crate chrono;
extern crate fs_extra;
extern crate indicatif;
extern crate maud;
extern crate pulldown_cmark;
extern crate serde;
extern crate serde_json;

pub mod diary;

use self::diary::{
    builder::BuilderOption, builder::DiaryBuilder, diary_builder::DiaryDayFilesBuilder, diary_page::DiaryPage, index_builder::IndexBuilder, parser::parse_daily, top_page_builder::TopPageBuilder,
};
use chrono::{Date, Local};
use fs_extra::dir::*;
use std::fs::{self, File};
use std::io;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

pub fn prepear_dir() -> io::Result<bool> {
    let mut result = false;
    if !Path::new("docs/").exists() {
        fs::create_dir("docs/")?;
        result = true;
    }
    if !Path::new("docs/static").exists() {
        fs::create_dir("docs/static")?;
        result = true;
    }
    if !Path::new("docs/pages").exists() {
        fs::create_dir("docs/pages")?;
        result = true;
    }
    Ok(result)
}

pub fn copy_static_files() -> io::Result<()> {
    let mut options = CopyOptions::new(); //Initialize default values for CopyOptions
    options.overwrite = true;
    for entry in fs::read_dir("static")? {
        let path = entry?.path();
        match copy(path, "docs/static", &options) {
            Ok(_d) => {}
            Err(e) => println!("Error: {}", e.to_string()),
        }
    }
    Ok(())
}

pub fn build(dest: &str) -> io::Result<()> {
    match prepear_dir() {
        Ok(true) => println!("|> create docs directory"),
        Err(e) => println!("Error: {}", e.to_string()),
        _ => (),
    }
    match copy_static_files() {
        Ok(()) => println!("|> copied css files"),
        Err(e) => println!("Error: {}", e.to_string()),
    }
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
    let bp: BuilderOption = BuilderOption { dest: dest };
    pb.finish_and_clear();
    build_by_builder(&DiaryDayFilesBuilder::new(&bp), &mut v);
    build_by_builder(&TopPageBuilder::new(&bp), &mut v);
    build_by_builder(&IndexBuilder::new(&bp), &mut v);
    Ok(())
}

fn build_by_builder<'a>(builder: &impl DiaryBuilder<'a>, diaries: &mut Vec<DiaryPage>) -> () {
    println!("|> build by {}", builder.builder_name());
    match builder.build(diaries) {
        Ok(()) => (),
        Err(e) => println!("Error {}", e.to_string()),
    }
    ()
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

pub fn create_templates(since: Date<Local>) -> io::Result<()> {
    let mut date = since;
    while date != Local::today() {
        date = date + chrono::Duration::days(1);
        match create_diary_template(date) {
            Ok(true) => println!(">>> Create Template on {}", date.format("%Y/%m/%d")),
            Ok(false) => {}
            Err(e) => println!("Error: {}", e.to_string()),
        }
    }
    Ok(())
}
