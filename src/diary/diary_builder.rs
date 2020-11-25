use diary::builder::{BuilderOption, DiaryBuilder};
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

impl<'a> DiaryBuilder<'a> for DiaryDayFilesBuilder<'a> {
    fn builder_name(&self) -> &'static str {
        "diary day file builder"
    }

    fn new(opt: &'a BuilderOption) -> DiaryDayFilesBuilder<'a> {
        DiaryDayFilesBuilder { option: opt }
    }

    fn build(&self, diaries: &mut Vec<DiaryPage>) -> io::Result<()> {
        let pb = ProgressBar::new(diaries.len() as u64);
        for i in 0..diaries.len() {
            let back = if i == 0 { None } else { diaries.get(i - 1) };
            let after = diaries.get(i + 1);
            match self.write_day_file(&diaries[i], back, after) {
                Ok(()) => pb.inc(1),
                Err(e) => println!("Error: {}", e.to_string()),
            }
        }
        pb.finish_and_clear();
        Ok(())
    }
}
