use diary::builder::{BuildOption, DiaryBuilder, DiaryBuilderGen};
use diary::diary_page::DiaryPage;
use fs_extra::dir::*;
use std::fs;
use std::io;
use std::path::Path;

pub struct StaticFileBuilder<'a> {
    option: &'a BuildOption<'a>,
}

impl<'a> DiaryBuilderGen<'a> for StaticFileBuilder<'a> {
    fn new(opt: &'a BuildOption) -> Self {
        StaticFileBuilder { option: opt }
    }
}

impl<'a> DiaryBuilder<'a> for StaticFileBuilder<'a> {
    fn builder_name(&self) -> &'static str {
        "static file builder"
    }

    fn build(&self, _: &mut std::vec::Vec<DiaryPage>) -> io::Result<()> {
        if !Path::new(self.option.dest).exists() {
            fs::create_dir(self.option.dest)?;
        }
        // staticディレクトリに配置しているものをそのままトップレベルに置きたいので、ファイルとディレクトリでコピーを変える必要がある
        for entry in fs::read_dir("static")? {
            let path = entry?.path();
            if path.is_dir() {
                let mut options = CopyOptions::new(); //Initialize default values for CopyOptions
                options.overwrite = true;
                match copy(path, self.option.dest, &options) {
                    Ok(_d) => {}
                    Err(e) => println!("Error: {}", e.to_string()),
                }
            } else {
                let mut options = fs_extra::file::CopyOptions::new();
                options.overwrite = true;
                match path.file_name() {
                    Some(name) => match fs_extra::file::copy(&path, "docs/".to_string() + name.to_str().expect("dummy"), &options) {
                        Ok(_d) => {}
                        Err(e) => println!("Error: {}", e.to_string()),
                    },
                    None => {}
                }
            }
        }
        Ok(())
    }
}
