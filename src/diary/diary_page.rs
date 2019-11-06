use crate::diary::components::page;
use chrono::{Date, Local};
use maud::{html, PreEscaped};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct DiaryPage {
    pub day: Date<Local>,
    pub title: String,
    pub content: String,
}

impl DiaryPage {
    pub fn get_url(&self) -> String {
        "https://diary.sh4869.net".to_string() + &self.day.format("/%Y/%m/%d").to_string() + &".html"
    }
    pub fn generate_html(&self, before: Option<&DiaryPage>, after: Option<&DiaryPage>) -> String {
        let title = self.day.format("%Y/%m/%d").to_string() + &" - " + &self.title;
        let markup = page(
            &title,
            html! {
                div.row.navigation {
                    div class=("col-xs-6")  {
                        @if after.is_some() {
                            @let link = "/".to_string() + &(after.unwrap().day.format("%Y/%m/%d").to_string()) + ".html";
                            time.small.diary {(after.unwrap().day.format("%Y/%m/%d"))}
                            div.day {
                                a href=(link) {
                                    p {(&after.unwrap().title)}
                                }
                            }
                        }
                    }
                    div class=("col-xs-6") {
                        @if before.is_some() {
                            @let link = "/".to_string() + &(before.unwrap().day.format("%Y/%m/%d").to_string()) + ".html";
                            time.small.diary {(before.unwrap().day.format("%Y/%m/%d"))}
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
                            time.diary {(self.day.format("%Y/%m/%d"))};
                            h1 {(self.title)};
                        }
                        div.daily {
                            (PreEscaped(&self.content))
                        }
                    }
                }
            },
        );
        return markup.into_string();
    }
}
