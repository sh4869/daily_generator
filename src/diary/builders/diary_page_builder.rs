use diary::builder::{BuildOption, DiaryBuilder, DiaryBuilderGen};
use diary::components::page;
use diary::diary_page::DiaryPage;
use maud::{html, PreEscaped};
use std::fs::{self, File};
use std::io;
use std::io::prelude::*;
use std::path::Path;

use indicatif::ProgressBar;

pub struct DiaryPageBuilder<'a> {
    option: &'a BuildOption<'a>,
}

impl DiaryPageBuilder<'_> {
    fn write_day_file(&self, target: &DiaryPage, before: Option<&DiaryPage>, after: Option<&DiaryPage>) -> io::Result<()> {
        let destpath = self.option.dest.to_string() + &target.get_path();
        let parent = Path::new(&destpath).parent().unwrap();
        if parent.exists() == false {
            fs::create_dir_all(parent.to_str().unwrap())?;
        }
        let mut file = File::create(&destpath)?;
        file.write_all(generate_html(target, before, after).as_bytes())?;
        Ok(())
    }
}

impl<'a> DiaryBuilderGen<'a> for DiaryPageBuilder<'a> {
    fn new(opt: &'a BuildOption) -> DiaryPageBuilder<'a> {
        DiaryPageBuilder { option: opt }
    }
}

impl<'a> DiaryBuilder<'a> for DiaryPageBuilder<'a> {
    fn builder_name(&self) -> &'static str {
        "diary day file builder"
    }

    fn build(&self, diaries: &mut Vec<DiaryPage>) -> io::Result<()> {
        let pb = ProgressBar::new(diaries.len() as u64);
        for i in 0..diaries.len() {
            match self.write_day_file(&diaries[i], if i == 0 { None } else { diaries.get(i - 1) }, diaries.get(i + 1)) {
                Ok(()) => pb.inc(1),
                Err(e) => println!("Error: {}", e.to_string()),
            }
        }
        pb.finish_and_clear();
        Ok(())
    }
}

fn generate_html(target: &DiaryPage, before: Option<&DiaryPage>, after: Option<&DiaryPage>) -> String {
    let title = target.day.format("%Y/%m/%d").to_string() + &" - " + &target.title;
    let markup = page(
        &title,
        true,
        html! {
            div.row.navigation {
                div class=("col-xs-6") {
                    @if before.is_some() {
                        time.small.diary {(before.unwrap().day.format("%Y/%m/%d"))}
                        div.day {
                            a href=(before.unwrap().get_path()) {
                                p {(&before.unwrap().title)}
                            }
                        }
                    }
                }
                div class=("col-xs-6")  {
                    @if after.is_some() {
                        time.small.diary {(after.unwrap().day.format("%Y/%m/%d"))}
                        div.day {
                            a href=(after.unwrap().get_path()) {
                                p {(&after.unwrap().title)}
                            }
                        }
                    }
                }
            }
            div.row {
                div class=("col-xs-12"){
                    div.info {
                        time.diary {(target.day.format("%Y/%m/%d"))};
                        h1 {(target.title)};
                    }
                    div.diary {
                        (PreEscaped(&target.content))
                    }
                }
            }
        },
    );
    return markup.into_string();
}
