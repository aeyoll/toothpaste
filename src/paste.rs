use chrono::NaiveDateTime;

#[crud_table]
#[derive(Debug, Clone)]
pub struct Paste {
    pub id: Option<u32>,
    pub filename: Option<String>,
    pub content: Option<String>,
    pub create_time: Option<NaiveDateTime>,
}
