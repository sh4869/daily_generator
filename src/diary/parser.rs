use chrono::{NaiveDate};
use diary::diary_page::DiaryPage;
use pulldown_cmark::{html, Options, Parser};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::path::{Path, MAIN_SEPARATOR};

const EMBEDLY_TAG: &str = r##"
<script async src="//cdn.embedly.com/widgets/platform.js"></script>
"##;

fn get_title(md: &String) -> io::Result<String> {
    let v: Vec<&str> = md.split("---").collect();
    if v.len() < 2 {
        return Err(Error::new(ErrorKind::InvalidData, "title not found"));
    }
    Ok((v[1].split("title:").collect::<Vec<&str>>())[1].trim().into())
}

fn get_date(filepath: &String) -> io::Result<NaiveDate> {
    let diarystr = filepath.clone().replace(".md", "");
    let diaryv: Vec<&str> = diarystr.split(MAIN_SEPARATOR).collect();
    let y = (diaryv[1].parse::<i32>().map_err(|err| Error::new(ErrorKind::InvalidData, err)))?;
    let m = (diaryv[2].parse::<u32>().map_err(|err| Error::new(ErrorKind::InvalidData, err)))?;
    let d = (diaryv[3].parse::<u32>().map_err(|err| Error::new(ErrorKind::InvalidData, err)))?;
    let date = NaiveDate::from_ymd_opt(y, m, d).ok_or(Error::new(ErrorKind::InvalidData, "error on invalid date"))?;
    Ok(date)
}

fn convert_markdown(md: &str) -> io::Result<String> {
    let parser = Parser::new_ext(&md, Options::all());
    let mut html_buf = String::new();
    html::push_html(&mut html_buf, parser);
    Ok(html_buf)
}

pub fn parse_diary(path: &Path) -> io::Result<DiaryPage> {
    let mut file = File::open(path)?;
    let date;
    match get_date(&path.to_str().unwrap().into()) {
        Ok(d) => date = d,
        Err(e) => {
            println!("{}", e.to_string());
            return Err(Error::new(ErrorKind::InvalidData, e.to_string()));
        }
    }
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;
    // タイトルの取得
    let title;
    match get_title(&mut file_content) {
        Ok(s) => title = if s.is_empty() { String::from("無題") } else { s },
        Err(e) => {
            return Err(Error::new(ErrorKind::InvalidData, e.to_string()));
        }
    };
    // 中身の取得 & Markdownの変換
    let md = file_content.splitn(3, "---").collect::<Vec<&str>>()[2];
    let content;
    match convert_markdown(&md) {
        Ok(md) => content = md,
        Err(e) => {
            println!("Error: {}", e.to_string());
            return Err(Error::new(ErrorKind::InvalidData, e.to_string()));
        }
    };
    let diary = DiaryPage {
        content: if content.find("class=\"embedly-card\"").is_some() {
            String::from(EMBEDLY_TAG) + &content
        } else {
            content
        },
        title: title,
        day: date,
    };
    Ok(diary)
}
