#![feature(proc_macro)]
extern crate pulldown_cmark;
extern crate maud;
extern crate chrono;
extern crate glob;

use std::io;
use std::io::{Error, ErrorKind};
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use pulldown_cmark::{html, Parser};
use maud::{html, PreEscaped};
use chrono::{Local, Date, TimeZone};

struct Daily {
    day: Date<Local>,
    title: String,
    content: String,
}

impl Daily {
    fn generate_html(&self) -> String {
        let css = r##"
<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/normalize/7.0.0/normalize.css"/>
<link rel="stylesheet" href="../../layers.min.css" />
<link rel="stylesheet" href="../../layers.section.min.css" />
<link rel="stylesheet" href="../../index.css"/>
<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.12.0/styles/hopscotch.min.css"/>
<script src="//cdnjs.cloudflare.com/ajax/libs/highlight.js/9.12.0/highlight.min.js"></script>
<script>hljs.initHighlightingOnLoad();</script>"##;
        let disqus = r##"
<div id="disqus_thread"></div>
<script>

/**
*  RECOMMENDED CONFIGURATION VARIABLES: EDIT AND UNCOMMENT THE SECTION BELOW TO INSERT DYNAMIC VALUES FROM YOUR PLATFORM OR CMS.
*  LEARN WHY DEFINING THESE VARIABLES IS IMPORTANT: https://disqus.com/admin/universalcode/#configuration-variables*/
/*
var disqus_config = function () {
this.page.url = PAGE_URL;  // Replace PAGE_URL with your page's canonical URL variable
this.page.identifier = PAGE_IDENTIFIER; // Replace PAGE_IDENTIFIER with your page's unique identifier variable
};
*/
(function() { // DON'T EDIT BELOW THIS LINE
var d = document, s = d.createElement('script');
s.src = 'https://diary-sh4869-net.disqus.com/embed.js';
s.setAttribute('data-timestamp', +new Date());
(d.head || d.body).appendChild(s);
})();
</script>
<noscript>Please enable JavaScript to view the <a href="https://disqus.com/?ref_noscript">comments powered by Disqus.</a></noscript>
                            "##;
        let title = self.day.format("%Y/%m/%d").to_string() + &" - " + &self.title;
        let markup =
            html! {
            html {
                head {
                    meta chaset="utf-8";
                    meta name="viewport" content="width=device-width, initial-scale=1";
                    (PreEscaped(css))
                    title (title)
                    "\n"
                }
                body{
                    div.row {
                        div.row-content.buffer {
                            div.column.twelve.top#header {
                                a href=("/") {
                                    img src="/logo.png";
                                }
                            }
                            div.clear;
                            div.info {
                                div.date {
                                    p (self.day.format("%Y/%m/%d"))
                                }
                                h1 (self.title);
                            }
                            div.daily {
                                (PreEscaped(&self.content))
                                div.signature {
                                    p ("Written by sh4869");
                                }
                                (PreEscaped(disqus))
                            }
                            footer {
                                hr;
                                a href=("/") "Daily Bread"
                                p (PreEscaped("&copy; 2017 <a href=\"http://sh4869.net\">sh4869</a>") )
                            }
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

fn get_date(filepath: &String) -> io::Result<Date<Local>> {
    let dailystr = filepath
        .clone()
        .replace("\\", "/")
        .replace(".md", "")
        .replace("diary/", "");
    let dailyv: Vec<&str> = dailystr.split("/").collect();
    let y = try!(dailyv[0].parse::<i32>().map_err(|err| {
        Error::new(ErrorKind::InvalidData, err)
    }));
    let m = try!(dailyv[1].parse::<u32>().map_err(|err| {
        Error::new(ErrorKind::InvalidData, err)
    }));
    let d = try!(dailyv[2].parse::<u32>().map_err(|err| {
        Error::new(ErrorKind::InvalidData, err)
    }));
    let date = Local.ymd(y, m, d);
    Ok(date)
}

fn convert_markdown(md: &str) -> io::Result<String> {
    let parser = Parser::new(&md);
    let mut html_buf = String::new();
    html::push_html(&mut html_buf, parser);
    Ok(html_buf)
}

fn write_day_file(daily: &Daily) -> io::Result<()> {
    let destpath = "docs/".to_string() + &daily.day.format("%Y/%m/%d").to_string() + &".html";
    let parent = Path::new(&destpath).parent().unwrap();
    if parent.exists() == false {
        fs::create_dir_all(parent.to_str().unwrap())?;
    }
    let mut file = File::create(&destpath)?;
    file.write_all(daily.generate_html().as_bytes())?;
    Ok(())
}

fn build_daily(path: &Path) -> io::Result<Daily> {
    let mut file = File::open(path)?;
    let date;
    match get_date(&path.to_str().unwrap().into()) {
        Ok(d) => date = d,
        Err(e) => {
            println!("{}", e.to_string());
            return Err(Error::new(ErrorKind::InvalidData, e.to_string()));
        }
    }
    let mut daily = Daily {
        content: "".into(),
        title: "".into(),
        day: date,
    };

    let mut content = String::new();
    file.read_to_string(&mut content)?;
    // タイトルの取得
    match get_title(&mut content) {
        Ok(s) => daily.title = s,
        Err(e) => println!("Error: {}", e.to_string()),
    }

    let md = content.split("---").collect::<Vec<&str>>()[2];
    match convert_markdown(&md) {
        Ok(md) => daily.content = md,
        Err(e) => println!("Error: {}", e.to_string()),
    }
    match write_day_file(&daily) {
        Ok(()) => {}
        Err(e) => println!("Error: {}", e.to_string()),
    }
    println!(">>>>> Build {}", daily.day.format("%Y/%m/%d"));
    Ok(daily)
}

fn build_top_page(dailies: &mut Vec<Daily>) -> io::Result<()> {
    dailies.sort_by(|a, b| b.day.cmp(&a.day));
    let css = r##"
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/normalize/7.0.0/normalize.css" />
    <link rel="stylesheet" href="layers.section.min.css" />
    <link rel="stylesheet" href="layers.min.css" />
    <link rel="stylesheet" href="index.css"/>
    "##;
    let markup =
        html! {
        head {
            meta chaset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1";
            (PreEscaped(css))
            title "Daily Bread"
        }
        body {
            div.row {
                div.row-content.buffer {
                    div.column.twelve.top#header {
                        a href=("/") {
                            img src="/logo.png";
                        }
                    }
                    div.clear;
                    @for (i,daily) in dailies.iter().enumerate() {
                        @let link = daily.day.format("%Y/%m/%d").to_string() + ".html";
                        @if i % 2 == 0 {
                            div.column.half {
                                div.day {
                                    a href=(link) {
                                        div.date {
                                            p (daily.day.format("%Y/%m/%d"))
                                        }
                                        h1 (daily.title)
                                    }
                                }
                            }
                        } @else {
                            div.column.half.last {
                                div.day {
                                    a href=(link) {
                                        div.date {
                                            p (daily.day.format("%Y/%m/%d"))
                                        }
                                        h1 (daily.title)
                                    }
                                }
                            }
                        }
                    }
                    footer {
                        a href=("/") "Daily Bread"
                        p (PreEscaped("&copy; 2017 <a href=\"http://sh4869.net\">sh4869</a>") )
                    }
                }
            }
        }
    };
    let mut file = File::create("docs/index.html")?;
    file.write_all(markup.into_string().as_bytes())?;
    Ok(())
}

fn build() -> io::Result<()> {
    let mut paths: Vec<PathBuf> = Vec::new();
    for entry in glob::glob("diary/**/*.md").map_err(|err| {
        Error::new(ErrorKind::InvalidData, err)
    })?
    {
        match entry {
            Ok(path) => paths.push(path),
            Err(e) => println!("{}", e.to_string()),
        }
    }
    let mut v: Vec<Daily> = Vec::new();
    for path in paths {
        match build_daily(path.as_path()) {
            Ok(daily) => v.push(daily),
            Err(e) => println!("{}", e),
        }
    }
    match build_top_page(&mut v) {
        Ok(()) => println!(">>> Build toppage"),
        Err(e) => println!("Error: {}", e.to_string()),
    }
    Ok(())
}

fn prepar_dir() -> io::Result<()> {
    if Path::new("docs/").exists() == false {
        fs::create_dir("docs/")?;
    }
    Ok(())
}

fn copy_css_image() -> io::Result<()> {
    fs::copy("src/css/index.css", "docs/index.css")?;
    fs::copy("src/css/layers.min.css", "docs/layers.min.css")?;
    fs::copy(
        "src/css/layers.section.min.css",
        "docs/layers.section.min.css",
    )?;
    fs::copy("src/img/logo.png", "docs/logo.png")?;
    Ok(())
}

fn main() {
    match prepar_dir() {
        Ok(()) => println!(">>> Create docs directory"),
        Err(e) => println!("Error: {}", e.to_string()),
    }
    match copy_css_image() {
        Ok(()) => println!(">>> Copied css files."),
        Err(e) => println!("Error: {}", e.to_string()),
    }
    match build() {
        Ok(()) => println!(">>> All Dailies build ended."),
        Err(e) => println!("Error: {}", e.to_string()),
    }
}
