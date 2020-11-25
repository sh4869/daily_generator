use diary::diary_page::DiaryPage;
use std::io;


pub struct BuilderOption<'a> {
    pub dest: &'a str,
    pub url: &'a str
}

pub trait DiaryBuilderGen<'a> {
    fn new(opt: &'a BuilderOption) -> Self;
}

pub trait DiaryBuilder<'a> {
    fn builder_name(&self) -> &'static str;

    fn build(&self, diaries: &mut Vec<DiaryPage>) -> io::Result<()>;
}
