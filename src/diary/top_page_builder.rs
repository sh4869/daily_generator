use crate::diary::components::page;
use crate::diary::diary_page::DiaryPage;
use diary::builder::{BuilderOption, DiaryBuilder, DiaryBuilderGen};
use maud::html;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

const PER_PAGE: i32 = 60;

pub struct TopPageBuilder<'a> {
    option: &'a BuilderOption<'a>,
}

impl<'a> DiaryBuilderGen<'a> for TopPageBuilder<'a> {
    fn new(opt: &'a BuilderOption) -> Self {
        TopPageBuilder { option: opt }
    }
}

impl<'a> DiaryBuilder<'a> for TopPageBuilder<'a> {
    fn builder_name(&self) -> &'static str {
        "top page builder"
    }
    fn build(&self, diaries: &mut Vec<DiaryPage>) -> io::Result<()> {
        if !Path::new(&(self.option.dest.to_string() + "/pages")).exists() {
            fs::create_dir(self.option.dest.to_string() + "/pages")?;
        }
        diaries.sort_by(|a, b| b.day.cmp(&a.day));
        diaries.retain(|daily| daily.title != "SKIP");
        let page_size = (diaries.len() as i32) / PER_PAGE + 1;
        for x in 0..page_size {
            let start = (x * PER_PAGE) as usize;
            let end = if x == page_size - 1 { diaries.len() } else { ((x + 1) * PER_PAGE) as usize };
            let markup = page(
                "Daily Bread",
                false,
                html! {
                    div.row {
                        @for daily in diaries.as_slice()[start..end].iter() {
                            @let link = daily.day.format("/%Y/%m/%d").to_string() + ".html";
                            div class=("col-xs-12 col-md-4") {
                                div.day_colum {
                                    time class=("diary") {(daily.day.format("%Y/%m/%d"))};
                                    a href=(link) {
                                        p.diary_title {(daily.title)}
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
                },
            );
            let filename = if x == 0 {
                self.option.dest.to_string() + &"/index.html".to_string()
            } else {
                format!("{}/pages/{}.html", self.option.dest, x + 1)
            };
            let mut file = File::create(filename)?;
            file.write_all(markup.into_string().as_bytes())?;
        }
        Ok(())
    }
}
