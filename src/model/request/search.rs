use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BookSearch {
    pub title: Option<String>,
    pub author: Option<String>,
    pub isbn: Option<String>,
}
