use chrono::NaiveDate;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct DiaryPage {
    pub day: NaiveDate,
    pub title: String,
    pub content: String,
}

impl DiaryPage {
    pub fn get_path(&self) -> String {
        self.day.format("/%Y/%m/%d").to_string() + &".html"
    }
}
