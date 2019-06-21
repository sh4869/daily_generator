use serde::{Serialize};
use std::io::prelude::*;
use std::fs::File;
use std::io;
use crate::diary::diary_page::DiaryPage;

#[derive(Serialize)]
struct IndexContent {
    title: String,
    url: String,
    body: String
}


pub fn build_index_json(dailes: &Vec<DiaryPage>) -> io::Result<()> {
    let index_contents = dailes.into_iter().map(|d| 
        IndexContent{
            title: String::from(d.clone().title),
            body:String::from(d.clone().content),
            url: String::from(d.clone().get_url())
        }
    ).collect::<Vec<_>>();
    let j = serde_json::to_string(&index_contents)?;
    let mut file = File::create("docs/index.json")?;
    file.write_all(j.as_bytes())?;
    Ok(())
}