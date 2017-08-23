#![feature(proc_macro)]
extern crate pulldown_cmark;
extern crate maud;
extern crate chrono;

use std::io;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::{Path,PathBuf};

use pulldown_cmark::{html, Parser};
use maud::{html, PreEscaped};
use chrono::{Local,Date,TimeZone};


struct Daily {
    day: Date<Local>,
    title: String,
    content: String,
}

impl Daily {
    fn generate_html(&self) -> String {
        let css = r##"
        <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/normalize/7.0.0/normalize.css" />
        <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/skeleton/2.0.4/skeleton.css" />
        "##;
        let markup =
            html! {
            html {
                head {
                    meta chaset="utf-8";
                    meta name="viewport" content="width=device-width, initial-scale=1";
                    (PreEscaped(css))
                    style type="text/css" {
                        ".title{text-align:center;}"
                        ".title h1{font-size: 4rem;}"
                        ".title h2{font-size: 3.5rem;}"
                        ".daily h1{font-size: 2.5rem;}"
                        ".daily p{text-indent:1em;}"
                    }
                    title {
                        (self.day) " " (self.title)
                    }
                }
                body{
                    div.container {
                        div.title {
                            h1 (self.day.format("%Y/%m/%d"));
                            h2 (self.title);
                            hr;
                        }
                        div.daily {
                            (PreEscaped(&self.content))
                        }
                    }   
                }
            }
        };
        return markup.into_string();
    }
}

fn get_title(md: &String) -> io::Result<String> {
    let v: Vec<&str> = md.split("---").collect();
    Ok(
        (v[1].split("title:").collect::<Vec<&str>>())[1]
            .trim()
            .into(),
    )
}

fn get_date(filepath: &String) -> Date<Local> {
    let dailystr = filepath.clone().replace("\\", "/").replace(".md", "");
    let dailyv: Vec<&str> = dailystr.split("/").collect();
    let date = Local.ymd(dailyv[0].parse::<i32>().unwrap(),dailyv[1].parse::<u32>().unwrap(),dailyv[2].parse::<u32>().unwrap());
    date
}

fn conver_md(path: &Path) -> io::Result<Daily> {
    let rootdir = fs::canonicalize(Path::new("."))?;
    let mut file = File::open(path)?;

    // FilePathを取得(ex: 2017/08/12.md)
    let mut filepath = path.to_str().unwrap().to_string();
    filepath = filepath.replace(
        &(rootdir.as_path().to_str().unwrap().to_string() + &"\\".to_string()),
        "",
    );

    let date = get_date(&filepath);

    let mut daily = Daily {
        content: "".into(),
        title: "".into(),
        day: date,
    };

    println!("Building {}",daily.day);

    // ファイルの中身を読み取る
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // タイトルの取得
    match get_title(&mut content) {
        Ok(s) => daily.title = s,
        Err(e) => println!("Error: {}", e.to_string()),
    }

    // Markdownの本文部分を取得する。
    let v: Vec<&str> = content.split("---").collect();
    let md = v[2];

    // MarkdownをParse、HTMLに変換
    let parser = Parser::new(&md);
    let mut html_buf = String::new();
    html::push_html(&mut html_buf, parser);
    daily.content = html_buf;

    // Make Directory, and Write Files
    if Path::new("dest/").exists() == false {
        fs::create_dir("dest/")?;
    }
    let destpath = "dest/".to_string() + &daily.day.format("%Y/%m/%d").to_string() + &".html";
    let parent = Path::new(&destpath).parent().unwrap();
    if parent.exists() == false {
        fs::create_dir_all(parent.to_str().unwrap())?;
    }
    let mut file = File::create(&destpath)?;
    file.write_all(daily.generate_html().as_bytes())?;
    Ok(daily)
}

/*
fn build_monthly_page(dailies: &mut Vec<Daily>){

    
}
*/

fn build_top_page(dailies: &mut Vec<Daily>) -> io::Result<()>{
    dailies.sort_by(|a,b| b.day.cmp(&a.day));
    let css = r##"
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/normalize/7.0.0/normalize.css" />
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/skeleton/2.0.4/skeleton.css" />
    "##;
    let markup = 
    html! {
        head {
            meta chaset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1";
            (PreEscaped(css))
            style type="text/css" {
                ".container{margin-top: 1em;margin-bottom: 1em;}"
                ".day{display:block;border-bottom: 1px solid black;}"
                ".day a{text-indent:1em;font-size:1.5rem;}"
                "p{margin-bottom: 0.5em;}"
            }
            title {
                "Daily Bread"
            }
        }
        body {
            div.container {
                @for daily in dailies.iter() {
                    div.day {
                        @let link = daily.day.format("%Y/%m/%d").to_string() + ".html";
                        p (daily.day.format("%Y/%m/%d"))
                        a href=(link){
                            p (daily.title)
                        }
                    }
                }
            }
        }
    };
    let destpath = "dest/index.html";
    let mut file = File::create(&destpath)?;
    //println!("{}",&markup.into_string());
    file.write_all(markup.into_string().as_bytes())?;
    Ok(())
}

#[allow(unused_must_use)]
fn visit_dirs(dir: &Path) -> io::Result<()> {
    let mut paths: Vec<PathBuf> = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                for entry in fs::read_dir(path)? {
                    let entry = entry?;
                    let path = entry.path();
                    paths.push(path);
                }
            } else {
                paths.push(path);
            }
        }
    }
    let mut v: Vec<Daily> = Vec::new();
    for path in paths {
        match conver_md(fs::canonicalize(path)?.as_path()) {
            Ok(daily) => v.push(daily),
            Err(e) => println!("{}",e)
        }
    }
    // build_monthly_page(&mut v);
    build_top_page(&mut v);
    Ok(())
}

fn build_daily(daily_path: &Path){
    match visit_dirs(daily_path) {
        Ok(()) => println!("All Dailies Build Ended."),
        Err(e) => println!("Error: {}", e.to_string()),
    }
}

fn main() {
    build_daily(Path::new("2017/"));
}
