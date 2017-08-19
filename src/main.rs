#![feature(proc_macro)]

extern crate pulldown_cmark;
extern crate maud;

use std::io;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;

use pulldown_cmark::{html, Parser};
use maud::{html, PreEscaped};

#[allow(unused_must_use)]
fn visit_dirs(dir: &Path, cb: &Fn(&Path) -> io::Result<()>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(fs::canonicalize(path)?.as_path());
            }
        }
    }
    Ok(())
}

struct Daily {
    day: String,
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
                        ".title h1{font-size: 5.6rem;}"
                        ".daily h1{font-size: 3.5rem;}"
                        ".daily p{text-indent:1em;}"
                    }
                    title {
                        (self.day) " " (self.title)
                    }
                }
                body{
                    div.container {
                        div.title {
                            h1 (self.day);
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

fn printmd(path: &Path) -> io::Result<()> {
    let rootdir = fs::canonicalize(Path::new("."))?;
    let mut file = File::open(path)?;

    // FilePathを取得(ex: 2017/08/12.md)
    let mut filepath = path.to_str().unwrap().to_string();
    filepath = filepath.replace(
        &(rootdir.as_path().to_str().unwrap().to_string() + &"\\".to_string()),
        "",
    );

    let mut daily = Daily {
        content: "".into(),
        title: "".into(),
        day: "".into(),
    };
    // 日時を決定
    daily.day = filepath.clone().replace("\\", "/").replace(".md", "");

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
    let destpath = "dest/".to_string() + &daily.day + &".html";
    let parent = Path::new(&destpath).parent().unwrap();
    if parent.exists() == false {
        fs::create_dir_all(parent.to_str().unwrap())?;
    }
    let mut file = File::create(&destpath)?;
    file.write_all(daily.generate_html().as_bytes())?;
    println!("{}", daily.generate_html());
    Ok(())
}

fn main() {
    let f = printmd;
    match visit_dirs(Path::new("2017/"), &f) {
        Ok(()) => println!("Good Bye"),
        Err(e) => println!("Error: {}", e.to_string()),
    }
}
