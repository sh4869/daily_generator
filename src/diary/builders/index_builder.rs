use crate::diary::diary_page::DiaryPage;
use chrono::Datelike;
use diary::builder::{BuildOption, DiaryBuilder, DiaryBuilderGen};
use serde::Serialize;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io;
use std::io::prelude::*;
use std::path::Path;

#[derive(Serialize)]
struct IndexContent {
    title: String,
    url: String,
    body: String,
}

pub struct IndexBuilder<'a> {
    option: &'a BuildOption<'a>,
}

impl<'a> DiaryBuilderGen<'a> for IndexBuilder<'a> {
    fn new(opt: &'a BuildOption) -> Self {
        IndexBuilder { option: opt }
    }
}

impl<'a> DiaryBuilder<'a> for IndexBuilder<'a> {
    fn builder_name(&self) -> &'static str {
        "index builder"
    }
    fn build(&self, diaries: &mut Vec<DiaryPage>) -> io::Result<()> {
        let path = self.option.dest.to_string() + &String::from("/indexes");
        if !Path::new(&path).exists() {
            fs::create_dir(&path)?;
        }
        let index_contents = diaries.into_iter().fold(
            HashMap::new(),
            |mut m: HashMap<u32, HashMap<String, IndexContent>>, d| {
                let c = IndexContent {
                    title: String::from(d.clone().title),
                    body: String::from(d.clone().content),
                    url: String::from(self.option.url.to_string() + &d.clone().get_path()),
                };
                m.entry(d.day.year_ce().1)
                    .or_insert_with(std::collections::HashMap::new)
                    .insert(d.day.format("%Y/%m/%d").to_string(), c);
                m
            },
        );
        for (key,value) in &index_contents {
            let j = serde_json::to_string(&value)?;
            let mut file = File::create(self.option.dest.to_string() + "/indexes/" + &key.to_string() + ".json")?;
            file.write_all(j.as_bytes())?;
        }
        Ok(())
    }
}
