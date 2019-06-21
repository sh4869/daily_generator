use chrono::{Date, Local};
use maud::{html, PreEscaped, DOCTYPE};

pub static HEADER_WORD: &str = "You will understand if you come here, You'll overlook your sleepiness";

pub struct DiaryPage {
    pub day: Date<Local>,
    pub title: String,
    pub content: String,
}

impl DiaryPage {
    pub fn generate_html(&self, before: Option<&DiaryPage>, after: Option<&DiaryPage>) -> String {
        let higlightjs = r##"<script src="//cdnjs.cloudflare.com/ajax/libs/highlight.js/9.12.0/highlight.min.js"></script><script>hljs.initHighlightingOnLoad();</script>"##;
        let csslist = [
            "https://cdnjs.cloudflare.com/ajax/libs/normalize/7.0.0/normalize.css",
            "/static/css/index.css",
            "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.12.0/styles/hopscotch.min.css",
            "https://fonts.googleapis.com/css?family=Noto+Sans+JP",
            "https://cdnjs.cloudflare.com/ajax/libs/flexboxgrid/6.3.1/flexboxgrid.min.css",
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
                        div#header class=("col-xs-12") {
                            a href=("/") {
                                h1.title {"Daily Bread"}
                            }
                            p {(HEADER_WORD)}
                        }
                    }
                    div.row.navigation {
                        div class=("col-xs-12 col-md-6")  {
                            @if after.is_some() {
                                @let link = "/".to_string() + &(after.unwrap().day.format("%Y/%m/%d").to_string()) + ".html";
                                time.small {(after.unwrap().day.format("%Y/%m/%d"))}
                                div.day {
                                    a href=(link) {
                                        p {(&after.unwrap().title)}
                                    }
                                }
                            }
                        }
                        div class=("col-xs-12 col-md-6") {
                            @if before.is_some() {
                                @let link = "/".to_string() + &(before.unwrap().day.format("%Y/%m/%d").to_string()) + ".html";
                                time.small {(before.unwrap().day.format("%Y/%m/%d"))}
                                div.day {
                                    a href=(link) {
                                        p {(&before.unwrap().title)}
                                    }
                                }
                            }
                        }
                    }
                    div.row {
                        div class=("col-xs-12"){
                            div.info {
                                time {(self.day.format("%Y/%m/%d"))};
                                h1 {(self.title)};
                            }
                            div.daily {
                                (PreEscaped(&self.content))
                            }
                        }
                    }
                    div.row {
                        div#footer class=("col-xs-12") {
                            hr.footer {}
                            p {(PreEscaped("&copy; 2017-2019 <a href=\"http://sh4869.net\">sh4869</a>") )}
                        }
                    }
                }
            }
        };
        return markup.into_string();
    }
}
