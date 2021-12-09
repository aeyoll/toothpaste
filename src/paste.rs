use chrono::NaiveDateTime;

use serde_aux::prelude::*;

#[crud_table]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Paste {
    pub id: Option<String>,
    pub filename: Option<String>,
    pub content: Option<String>,
    pub create_time: Option<NaiveDateTime>,

    // The number of seconds before deletion
    pub expire_after: Option<u32>,

    // The expire time
    pub expire_time: Option<NaiveDateTime>,

    // Should the paste appear in the homepage?
    #[serde(
        default = "bool::default",
        deserialize_with = "deserialize_bool_from_anything"
    )]
    pub private: bool,
}
