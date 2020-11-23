use diary::diary_page::DiaryPage;
use std::fs::{self, File};
use std::io;
use std::io::prelude::*;
use std::path::Path;

use indicatif::ProgressBar;

fn write_day_file(daily: &DiaryPage, before: Option<&DiaryPage>, after: Option<&DiaryPage>) -> io::Result<()> {
    let destpath = "docs/".to_string() + &daily.day.format("%Y/%m/%d").to_string() + &".html";
    let parent = Path::new(&destpath).parent().unwrap();
    if parent.exists() == false {
        fs::create_dir_all(parent.to_str().unwrap())?;
    }
    let mut file = File::create(&destpath)?;
    file.write_all(daily.generate_html(before, after).as_bytes())?;
    Ok(())
}

pub fn build_dailies(dailies: &mut Vec<DiaryPage>) -> io::Result<()> {
    let pb = ProgressBar::new(dailies.len() as u64);
    for i in 0..dailies.len() {
        let back = if i == 0 { None } else { dailies.get(i - 1) };
        let after = dailies.get(i + 1);
        match write_day_file(&dailies[i], back, after) {
            Ok(()) => pb.inc(1),
            Err(e) => println!("Error: {}", e.to_string()),
        }
    }
    pb.finish_and_clear();
    Ok(())
}
