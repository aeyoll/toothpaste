#[crud_table]
#[derive(Clone)]
pub struct Paste {
    pub id: Option<u32>,
    pub filename: Option<String>,
    pub content: Option<String>,
}
