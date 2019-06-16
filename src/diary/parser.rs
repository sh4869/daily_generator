use chrono::{Date, Local, TimeZone};
use diary::common::Daily;
use pulldown_cmark::{html, Options, Parser};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::path::{Path, MAIN_SEPARATOR};

fn get_title(md: &String) -> io::Result<String> {
    let v: Vec<&str> = md.split("---").collect();
    if v.len() < 2 {
        return Err(Error::new(ErrorKind::InvalidData, "title not found"));
    }
    Ok((v[1].split("title:").collect::<Vec<&str>>())[1].trim().into())
}

fn get_date(filepath: &String) -> io::Result<Date<Local>> {
    let dailystr = filepath.clone().replace(".md", "");
    let dailyv: Vec<&str> = dailystr.split(MAIN_SEPARATOR).collect();
    let y = try!(dailyv[1].parse::<i32>().map_err(|err| Error::new(ErrorKind::InvalidData, err)));
    let m = try!(dailyv[2].parse::<u32>().map_err(|err| Error::new(ErrorKind::InvalidData, err)));
    let d = try!(dailyv[3].parse::<u32>().map_err(|err| Error::new(ErrorKind::InvalidData, err)));
    let date = Local.ymd(y, m, d);
    Ok(date)
}

fn convert_markdown(md: &str) -> io::Result<String> {
    let parser = Parser::new_ext(&md, Options::all());
    let mut html_buf = String::new();
    html::push_html(&mut html_buf, parser);
    Ok(html_buf)
}

pub fn parse_daily(path: &Path) -> io::Result<Daily> {
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
        Ok(s) => title = s,
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
    let daily = Daily {
        content: content,
        title: title,
        day: date,
    };
    print!(">>>>> Parse {}\r", daily.day.format("%Y/%m/%d"));
    Ok(daily)
}
