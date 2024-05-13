use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

pub type Doi = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Literature {
    pub doi: Doi,
    pub title: String,
    pub author: String,
    pub date: Option<NaiveDate>,
    pub abstract_: Option<String>,
    pub refs: Vec<Ref>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ref {
    pub doi: Doi,
    pub date: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BriefLiterature {
    pub doi: Doi,
    pub title: String,
    pub author: String,
    pub year: Option<i32>,
    pub refs: Vec<Doi>,
}

impl From<Literature> for BriefLiterature {
    fn from(lit: Literature) -> Self {
        Self {
            doi: lit.doi,
            title: lit.title,
            author: lit.author,
            year: lit.date.map(|date| date.year()),
            refs: lit.refs.iter().map(|ref_| ref_.doi.clone()).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiteratureRet {
    pub doi: String,
    pub title: String,
    pub author: String,
    pub year: Option<i32>,
    pub refs: Vec<String>,
    pub score: f64,
}
