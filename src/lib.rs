#![feature(proc_macro_hygiene)]
extern crate chrono;
extern crate fs_extra;
extern crate indicatif;
extern crate maud;
extern crate pulldown_cmark;
extern crate rss;
extern crate serde;
extern crate serde_json;
extern crate voca_rs;

pub mod diary;

use self::diary::{
    builder::BuilderOption, builder::DiaryBuilder, builder::DiaryBuilderGen, diary_builder::DiaryDayFilesBuilder, index_builder::IndexBuilder, parser::parse_daily, rss_builder::RssBuilder,
    top_page_builder::TopPageBuilder,
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
    if !Path::new("docs/pages").exists() {
        fs::create_dir("docs/pages")?;
        result = true;
    }
    Ok(result)
}

pub fn copy_static_files() -> io::Result<()> {
    for entry in fs::read_dir("static")? {
        let path = entry?.path();
        if path.is_dir() {
            let mut options = CopyOptions::new(); //Initialize default values for CopyOptions
            options.overwrite = true;
            match copy(path, "docs/", &options) {
                Ok(_d) => {}
                Err(e) => println!("Error: {}", e.to_string()),
            }
        } else {
            let mut options = fs_extra::file::CopyOptions::new();
            options.overwrite = true;
            match path.file_name() {
                Some(name) => match fs_extra::file::copy(&path, "docs/".to_string() + name.to_str().expect("dummy"), &options) {
                    Ok(_d) => {}
                    Err(e) => println!("Error: {}", e.to_string()),
                },
                None => {}
            }
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
    pb.finish_and_clear();
    let bp: BuilderOption = BuilderOption {
        dest: &dest,
        url: "https://diary.sh4869.net",
    };
    let vec: Vec<Box<dyn DiaryBuilder>> = vec![
        Box::new(DiaryDayFilesBuilder::new(&bp)),
        Box::new(TopPageBuilder::new(&bp)),
        Box::new(IndexBuilder::new(&bp)),
        Box::new(RssBuilder::new(&bp)),
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
