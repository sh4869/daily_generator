use chrono::{Date, Local};
use maud::{html, PreEscaped, DOCTYPE};

pub struct Daily {
    pub day: Date<Local>,
    pub title: String,
    pub content: String,
}

impl Daily {
    pub fn generate_html(&self, before: Option<&Daily>, after: Option<&Daily>) -> String {
        let higlightjs = r##"<script src="//cdnjs.cloudflare.com/ajax/libs/highlight.js/9.12.0/highlight.min.js"></script><script>hljs.initHighlightingOnLoad();</script>"##;
        let csslist = [
            "https://cdnjs.cloudflare.com/ajax/libs/normalize/7.0.0/normalize.css",
            "/static/css/layers.min.css",
            "/static/css/layers.section.min.css",
            "/static/css/index.css",
            "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.12.0/styles/hopscotch.min.css",
            "https://fonts.googleapis.com/css?family=Noto+Sans+JP",
        ];
        let title = self.day.format("%Y/%m/%d").to_string() + &" - " + &self.title;
        let markup = html! {
            (DOCTYPE)
            html lang="ja" {
                head {
                    meta chaset="utf-8";
                    meta name="viewport" content="width=device-width, initial-scale=1";
                    @for url in &csslist {
                        link rel="stylesheet" href=(url);
                    }
                    (PreEscaped(higlightjs))
                    title {(title)}
                }
                body{
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
                            div.info {
                                time {(self.day.format("%Y/%m/%d"))};
                                h1 {(self.title)};
                            }
                            div.daily {
                                (PreEscaped(&self.content))
                            }
                            footer {
                                hr.footer;
                                div.row {
                                    div.clear {
                                    }
                                    div.row-content {
                                        div.column.small-full.medium-half.large-half  {
                                            @if after.is_some() {
                                                @let link = "/".to_string() + &(after.unwrap().day.format("%Y/%m/%d").to_string()) + ".html";
                                                time {(after.unwrap().day.format("%Y/%m/%d"))}
                                                div.day {
                                                    a href=(link) {
                                                        p {(&after.unwrap().title)}
                                                    }
                                                }
                                            }
                                        }
                                        div.column.small-full.medium-half.medium-last {
                                            @if before.is_some() {
                                                @let link = "/".to_string() + &(before.unwrap().day.format("%Y/%m/%d").to_string()) + ".html";
                                                time {(before.unwrap().day.format("%Y/%m/%d"))}
                                                div.day {
                                                    a href=(link) {
                                                        p {(&before.unwrap().title)}
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                p {(PreEscaped("&copy; 2017-2019 <a href=\"http://sh4869.net\">sh4869</a>") )}

                            }
                        }
                    }
                }
            }
        };
        return markup.into_string();
    }
}
