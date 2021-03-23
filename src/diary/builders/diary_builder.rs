use diary::builder::{BuilderOption, DiaryBuilder, DiaryBuilderGen};
use diary::diary_page::DiaryPage;
use std::fs::{self, File};
use std::io;
use std::io::prelude::*;
use std::path::Path;

use indicatif::ProgressBar;

pub struct DiaryDayFilesBuilder<'a> {
    option: &'a BuilderOption<'a>,
}

impl DiaryDayFilesBuilder<'_> {
    fn write_day_file(&self, daily: &DiaryPage, before: Option<&DiaryPage>, after: Option<&DiaryPage>) -> io::Result<()> {
        let destpath = self.option.dest.to_string() + &daily.day.format("/%Y/%m/%d").to_string() + &".html";
        let parent = Path::new(&destpath).parent().unwrap();
        if parent.exists() == false {
            fs::create_dir_all(parent.to_str().unwrap())?;
        }
        let mut file = File::create(&destpath)?;
        file.write_all(daily.generate_html(before, after).as_bytes())?;
        Ok(())
    }
}

impl<'a> DiaryBuilderGen<'a> for DiaryDayFilesBuilder<'a> {
    fn new(opt: &'a BuilderOption) -> DiaryDayFilesBuilder<'a> {
        DiaryDayFilesBuilder { option: opt }
    }
}

impl<'a> DiaryBuilder<'a> for DiaryDayFilesBuilder<'a> {
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
