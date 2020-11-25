use diary::diary_page::DiaryPage;
use std::io;

pub struct BuilderOption<'a> {
    pub dest: &'a str,
}

pub trait DiaryBuilder<'a> {
    fn builder_name(&self) -> &'static str;

    fn new(opt: &'a BuilderOption) -> Self;

    fn build(&self, diaries: &mut Vec<DiaryPage>) -> io::Result<()>;
}
