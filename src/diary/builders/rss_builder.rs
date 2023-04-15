use chrono::{DateTime, Utc};
use diary::builder::{BuildOption, DiaryBuilder, DiaryBuilderGen};
use diary::diary_page::DiaryPage;
use rss::{ChannelBuilder, GuidBuilder, ItemBuilder};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use voca_rs::strip::strip_tags;

pub struct RssBuilder<'a> {
    option: &'a BuildOption<'a>,
}

impl<'a> DiaryBuilderGen<'a> for RssBuilder<'a> {
    fn new(opt: &'a BuildOption) -> Self {
        RssBuilder { option: opt }
    }
}

impl<'a> DiaryBuilder<'a> for RssBuilder<'a> {
    fn builder_name(&self) -> &'static str {
        "rss builder"
    }

    fn build(&self, diaries: &mut Vec<DiaryPage>) -> io::Result<()> {
        diaries.sort_by(|a, b| b.day.cmp(&a.day));
        let items = diaries[0..(if diaries.len() > 100 { 100 } else { diaries.len() })]
            .into_iter()
            .map(|v| {
                ItemBuilder::default()
                    .title(v.clone().title)
                    .link(self.option.url.to_string() + &v.clone().get_path())
                    .guid(GuidBuilder::default().value(self.option.url.to_string() + &v.clone().get_path()).permalink(true).build().unwrap())
                    .pub_date(
                        v.clone()
                            .day
                            .and_hms_opt(23, 0, 0)
                            .map(|r| DateTime::<Utc>::from_utc(r, Utc).to_rfc2822())
                            .unwrap_or("default".to_string()),
                    )
                    .description(strip_tags(&v.clone().content))
                    .build()
                    .unwrap()
            })
            .collect::<Vec<_>>();
        let channel = ChannelBuilder::default()
            .title("Daily Bread")
            .link(self.option.url)
            .description("diary of @sh4869")
            .last_build_date(items[0].pub_date().unwrap_or("Wed, 17 Sep 1997 00:00:00 +0900").to_string())
            .items(items)
            .build()
            .unwrap();
        let mut file = File::create(self.option.dest.to_string() + "/rss.xml")?;
        file.write_all(channel.to_string().as_bytes())?;
        Ok(())
    }
}
