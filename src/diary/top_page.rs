use diary::common::Daily;
use maud::{html, PreEscaped, DOCTYPE};
use std::fs::{File};
use std::io;
use std::io::prelude::*;

pub fn build_top_page(dailies: &mut Vec<Daily>) -> io::Result<()> {
    dailies.sort_by(|a, b| b.day.cmp(&a.day));
    dailies.retain(|daily| daily.title != "SKIP");
    let csslist = [
        "https://cdnjs.cloudflare.com/ajax/libs/normalize/7.0.0/normalize.css",
        "/static/css/layers.min.css",
        "/static/css/layers.section.min.css",
        "/static/css/index.css",
        "https://fonts.googleapis.com/css?family=Noto+Sans+JP",
    ];
    let markup = html! {
        (DOCTYPE)
        html lang="ja" {
            head {
                meta chaset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                @for url in &csslist {
                    link rel="stylesheet" href=(url);
                }
                title {"Daily Bread"}
            }
            body {
                div.row {
                    div.row-content.buffer {
                        div.column.twelve.top#header {
                            a href=("/") {
                                h1.title {"Daily Bread"}
                            }
                            p { "It's alright , I remember sometimes the time we chose what to bring on the journey" }
                        }
                        div.clear {

                        }
                        @for (i,daily) in dailies.iter().enumerate() {
                            @let link = daily.day.format("%Y/%m/%d").to_string() + ".html";
                            @if i % 2 == 0 {
                                div.column.small-full.medium-half.large-half {
                                    div.day {
                                        time {(daily.day.format("%Y/%m/%d"))};
                                        div {
                                            a href=(link) {
                                                h2 {(daily.title)}
                                            }
                                        }
                                    }
                                }
                            } @else {
                                div.column.small-full.medium-half.medium-last {
                                    div.day {
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
                        footer {
                            a href=("/") {"Daily Bread"}
                            p {(PreEscaped("&copy; 2017 <a href=\"http://sh4869.net\">sh4869</a>") )}
                        }
                    }
                }
            }
        }
    };
    let mut file = File::create("docs/index.html")?;
    file.write_all(markup.into_string().as_bytes())?;
    Ok(())
}
