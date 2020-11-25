use diary::builder::{BuilderOption, DiaryBuilder};
use diary::diary_page::DiaryPage;
use rss::ChannelBuilder;
use rss::GuidBuilder;
use rss::ItemBuilder;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use voca_rs::strip::strip_tags;

pub struct RssBuilder<'a> {
    option: &'a BuilderOption<'a>,
}

impl<'a> DiaryBuilder<'a> for RssBuilder<'a> {
    fn builder_name(&self) -> &'static str {
        "rss builder"
    }
    fn new(opt: &'a BuilderOption) -> Self {
        RssBuilder { option: opt }
    }
    fn build(&self, diaries: &mut Vec<DiaryPage>) -> io::Result<()> {
        diaries.sort_by(|a, b| b.day.cmp(&a.day));
        let items = diaries[0..100]
            .into_iter()
            .map(|v| {
                ItemBuilder::default()
                    .title(v.clone().title)
                    .link(v.clone().day.format("https://diary.sh4869.net/%Y/%m/%d.html").to_string())
                    .guid(
                        GuidBuilder::default()
                            .value(v.clone().day.format("https://diary.sh4869.net/%Y/%m/%d.html").to_string())
                            .permalink(true)
                            .build()
                            .unwrap(),
                    )
                    .pub_date(v.clone().day.and_hms(23, 0, 0).to_rfc2822())
                    .description(strip_tags(&v.clone().content))
                    .build()
                    .unwrap()
            })
            .collect::<Vec<_>>();
        let channel = ChannelBuilder::default()
            .title("Daily Bread")
            .link("https://diary.sh4869.net")
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
