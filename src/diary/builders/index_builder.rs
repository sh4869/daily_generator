use crate::diary::diary_page::DiaryPage;
use diary::builder::{BuildOption, DiaryBuilder, DiaryBuilderGen};
use serde::Serialize;
use std::fs::File;
use std::io;
use std::io::prelude::*;

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
        let index_contents = diaries
            .into_iter()
            .map(|d| IndexContent {
                title: String::from(d.clone().title),
                body: String::from(d.clone().content),
                url: String::from(self.option.url.to_string() + &d.clone().get_path()),
            })
            .collect::<Vec<_>>();
        let j = serde_json::to_string(&index_contents)?;
        let mut file = File::create(self.option.dest.to_string() + "/index.json")?;
        file.write_all(j.as_bytes())?;
        Ok(())
    }
}
