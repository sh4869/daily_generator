use diary::diary_page::{DiaryPage,HEADER_WORD};
use maud::{html, PreEscaped, DOCTYPE};
use std::fs::File;
use std::io;
use std::io::prelude::*;

const PER_PAGE: i32 = 60;

const CSSLIST: [&str; 4] = [
    "https://cdnjs.cloudflare.com/ajax/libs/normalize/7.0.0/normalize.css",
    "/static/css/index.css",
    "https://fonts.googleapis.com/css?family=Noto+Sans+JP",
    "https://cdnjs.cloudflare.com/ajax/libs/flexboxgrid/6.3.1/flexboxgrid.min.css",
];

pub fn build_top_page(dailies: &mut Vec<DiaryPage>) -> io::Result<()> {
    dailies.sort_by(|a, b| b.day.cmp(&a.day));
    dailies.retain(|daily| daily.title != "SKIP");
    let page_size = (dailies.len() as i32) / PER_PAGE + 1;
    for x in 0..page_size {
        let start = (x * PER_PAGE) as usize;
        let end = if x == page_size - 1 {dailies.len()} else { ((x + 1) * PER_PAGE) as usize };
        let markup = html! {
            (DOCTYPE)
            html lang="ja" {
                head {
                    meta chaset="utf-8";
                    meta name="viewport" content="width=device-width, initial-scale=1";
                    @for url in &CSSLIST {
                        link rel="stylesheet" href=(url);
                    }
                    title {"Daily Bread"}
                }
                body {
                    div.row {
                        div#header class=("col-xs-12 top") {
                            a href=("/") {
                                h1.title {"Daily Bread"}
                            }
                            p {(HEADER_WORD)}
                        }
                    }
                    div.row {
                        @for daily in dailies.as_slice()[start..end].iter() {
                            @let link = daily.day.format("/%Y/%m/%d").to_string() + ".html";
                            div class=("col-xs-12 col-md-6") {
                                div.day_colum {
                                    time {(daily.day.format("%Y/%m/%d"))};
                                    div {
                                        a href=(link) {
                                            h2 {(daily.title)}
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div.row {
                        @for y in 0..page_size {
                            div class=("col-xs paging") {
                                @if y != x {
                                    @if y == 0 {
                                        p {
                                            a href=("/") { "1" }
                                        }
                                    } @else {
                                        p {
                                            a href=(format!("/pages/{}.html",(y+1))){ ((y+1).to_string()) }
                                        }
                                    }
                                } @else {
                                    p.current { ((y+1).to_string()) }
                                }
                            }
                        }
                    }
                    div.row {
                        div#footer class=("col-xs-12") {
                            footer {
                                p {(PreEscaped("&copy; 2017-2019 <a href=\"http://sh4869.net\">sh4869</a>") )}
                            }
                        }
                    }
                }
            }
        };
        let filename = if x == 0 { "docs/index.html".to_string() } else { format!("docs/pages/{}.html", x + 1) };
        let mut file = File::create(filename)?;
        file.write_all(markup.into_string().as_bytes())?;
    }
    Ok(())
}
