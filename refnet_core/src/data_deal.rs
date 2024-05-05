use std::collections::VecDeque;

use hashbrown::HashSet;
use itertools::Itertools;
use tokio::task;

use crate::{
    types::{Doi, Literature},
    CONCURRENCY,
};

use self::sql::insert_literature;

mod crossref;
mod sql;

pub async fn query_doi(doi: &str) -> Option<Literature> {
    // query database at first
    let literature = sql::get_literature(doi).await;
    if let Some(literature) = literature {
        return Some(literature);
    }

    // query crossref
    let cloned_doi = doi.to_owned(); // Clone the `doi` variable
    let literature = task::spawn_blocking(move || crossref::query_doi(&cloned_doi, crate::YEAR)) // Use the cloned `doi` variable in the closure
        .await
        .unwrap();

    // insert_literature(literature.clone()).await.unwrap();
    if let Some(literature) = &literature {
        insert_literature(literature.clone()).await.unwrap();
    }
    literature
}

pub async fn query_dois(dois: &Vec<Doi>) -> Vec<Literature> {
    // let mut lits = vec![];
    let join_handles = dois
        .into_iter()
        .map(|doi| {
            let doi = doi.to_string();
            tokio::task::spawn(async move { query_doi(&doi).await })
        })
        .collect::<Vec<_>>();

    let mut lits = Vec::new();
    for handle in join_handles {
        let lit = handle.await.unwrap();
        if let Some(lit) = lit {
            lits.push(lit);
        }
    }

    lits
}

pub async fn extend(doi: Doi, max_counts: usize) -> Vec<Literature> {
    let mut lits = vec![];
    let mut dois = VecDeque::new();
    dois.push_back(doi);
    let mut queried_dois = HashSet::new();
    let mut counter = 0;

    while counter < max_counts {
        // split ad CUNCURRENCY
        if dois.len() == 0 {
            break;
        }

        println!("{dois:?}");

        let dois_to_query: Vec<Doi> = dois
            .drain(..CONCURRENCY.min(dois.len()))
            .map(|doi| {
                queried_dois.insert(doi.clone());
                doi
            })
            .collect();

        counter += dois_to_query.len();
        let new_lits = query_dois(&dois_to_query).await;
        lits.extend(new_lits.clone());

        let new_dois = collect_ref_dois(&new_lits);
        let new_dois: Vec<Doi> = new_dois
            .into_iter()
            .unique()
            .take(max_counts - counter)
            .filter(|doi| !queried_dois.contains(doi))
            .collect();

        dois.extend(new_dois);
    }

    lits
}

fn collect_ref_dois(lits: &Vec<Literature>) -> Vec<Doi> {
    lits.iter()
        .flat_map(|lit| lit.refs.iter().map(|ref_| ref_.doi.clone()))
        .collect()
}
