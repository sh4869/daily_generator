use crate::diary::components::page;
use crate::diary::diary_page::DiaryPage;
use maud::html;
use std::fs::File;
use std::io;
use std::io::prelude::*;

const PER_PAGE: i32 = 40;

pub fn build_top_page(dailies: &mut Vec<DiaryPage>) -> io::Result<()> {
    dailies.sort_by(|a, b| b.day.cmp(&a.day));
    dailies.retain(|daily| daily.title != "SKIP");
    let page_size = (dailies.len() as i32) / PER_PAGE + 1;
    for x in 0..page_size {
        let start = (x * PER_PAGE) as usize;
        let end = if x == page_size - 1 { dailies.len() } else { ((x + 1) * PER_PAGE) as usize };
        let markup = page(
            "Daily Bread",
            html! {
                div.row {
                    @for daily in dailies.as_slice()[start..end].iter() {
                        @let link = daily.day.format("/%Y/%m/%d").to_string() + ".html";
                        div class=("col-xs-12 col-md-6") {
                            div.day_colum {
                                time class=("diary") {(daily.day.format("%Y/%m/%d"))};
                                a href=(link) {
                                    h2 {(daily.title)}
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
        let filename = if x == 0 { "docs/index.html".to_string() } else { format!("docs/pages/{}.html", x + 1) };
        let mut file = File::create(filename)?;
        file.write_all(markup.into_string().as_bytes())?;
    }
    Ok(())
}
