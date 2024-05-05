use chrono::NaiveDate;
use crossref::Crossref;

use crate::types::{Literature, Ref};

fn create_client() -> Crossref {
    let client = Crossref::builder()
        .proxy("http://127.0.0.1:7890")
        .polite("dsywh123@gmail.com")
        .build()
        .unwrap();

    client
}

pub fn query_doi(doi: &str, current_year: i32) -> Option<Literature> {
    let client = create_client();
    let work = match client.work(doi) {
        Ok(work) => work,
        Err(e) => {
            dbg!((doi, e));
            return None;
        }
    };

    let refs = work
        .reference
        .map(|refs| {
            refs.iter()
                .filter_map(|ref_| {
                    ref_.doi.clone().map(|doi| {
                        let year = ref_.year.clone();
                        Ref {
                            doi: doi,
                            date: year.map(|year| {
                                let mut year = year.parse().unwrap_or(1980);
                                if year > current_year {
                                    year = 1980
                                }
                                NaiveDate::from_ymd_opt(year, 1, 1).unwrap()
                            }),
                        }
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    let title = work.title.get(0).map(|s| s.to_owned()).unwrap_or_default();

    let author: String = work
        .author
        .map(|authors| {
            authors
                .iter()
                .map(|contributor| {
                    let given = contributor.given.clone().unwrap_or_default();
                    let family = contributor.family.clone().unwrap_or_default();
                    format!("{} {}", given, family)
                })
                .next()
        })
        .flatten()
        .unwrap_or_default();

    let date = work.issued.date_parts.0.get(0).map(|date| {
        NaiveDate::from_ymd_opt(
            date.get(0).cloned().flatten().unwrap_or(1980) as i32,
            date.get(1).cloned().flatten().unwrap_or(1),
            date.get(2).cloned().flatten().unwrap_or(1),
        )
        .unwrap()
    });
    // let date

    let literature = Literature {
        doi: doi.to_string(),
        title,
        author,
        date,
        abstract_: work.abstract_,
        refs,
    };

    Some(literature)
}
