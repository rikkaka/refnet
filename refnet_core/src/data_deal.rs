use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use cached::proc_macro::cached;
use flume::{Receiver, Sender};
use hashbrown::HashSet;
use itertools::Itertools;
use tokio::task;

use crate::types::{Doi, Literature};

use self::sql::insert_literature;

mod crossref;
mod sql;

async fn query_doi_sql(doi: &str) -> Option<Literature> {
    sql::get_literature(doi).await
}

async fn query_doi_crossref(doi: Doi) -> Option<Literature> {
    let literature = task::spawn_blocking(move || crossref::query_doi(&doi, crate::YEAR))
        .await
        .unwrap();

    if let Some(literature) = &literature {
        insert_literature(literature.clone()).await.unwrap();
    }
    literature
}

pub async fn query_doi(doi: &str) -> Option<Literature> {
    // query database at first
    let literature = query_doi_sql(doi).await;
    if literature.is_some() {
        return literature;
    }

    // query crossref
    let doi = Doi::from(doi);
    query_doi_crossref(doi).await
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

async fn query_doi_crossref_worker(
    doi_rx: Receiver<Doi>,
    lit_tx: Sender<Option<Literature>>,
    working_workers_counter: Arc<AtomicUsize>,
) {
    while let Ok(doi) = doi_rx.recv_async().await {
        println!("querying doi: {}", doi);
        working_workers_counter.fetch_add(1, Ordering::Relaxed);
        let lit = query_doi(&doi).await;
        // lit_tx.send(lit).unwrap();
        match lit_tx.send(lit) {
            Ok(_) => {}
            Err(_) => {
                break;
            }
        }
        working_workers_counter.fetch_sub(1, Ordering::Relaxed);
    }
}

#[cached(size = 100)]
pub async fn extend(doi: Doi, max_counts: usize, workers: usize) -> Vec<Literature> {
    let mut lits = vec![];
    let mut queried_dois: HashSet<Doi> = HashSet::new();

    let (doi_tx, doi_rx) = flume::unbounded();
    let (lit_tx, lit_rx) = flume::unbounded();
    let working_workers_counter = Arc::new(AtomicUsize::new(0));

    for _ in 0..workers {
        let doi_rx = doi_rx.clone();
        let lit_tx = lit_tx.clone();
        let working_workers_counter = Arc::clone(&working_workers_counter);
        tokio::spawn(async move {
            query_doi_crossref_worker(doi_rx, lit_tx, working_workers_counter).await;
        });
    }

    queried_dois.insert(doi.clone());
    doi_tx.send(doi).unwrap();

    while let Ok(lit) = lit_rx.recv_async().await {
        let new_dois: Vec<Doi> = lit
            .map(|lit| {
                lits.push(lit.clone());
                lit.refs
                    .iter()
                    .map(|ref_| ref_.doi.clone())
                    .unique()
                    .filter(|doi| !queried_dois.contains(doi))
                    .take(
                        (max_counts as i32
                            - lits.len() as i32
                            - working_workers_counter.load(Ordering::Relaxed) as i32)
                            as usize,
                    )
                    .collect()
            })
            .unwrap_or_default();

        if lits.len() >= max_counts || new_dois.len() == 0 && working_workers_counter.load(Ordering::Relaxed) == 0 {
            break;
        }

        queried_dois.extend(new_dois.clone());
        for doi in new_dois {
            doi_tx.send(doi).unwrap();
        }
    }

    lits

    // while counter < max_counts {
    //     // split ad CUNCURRENCY
    //     if dois.len() == 0 {
    //         break;
    //     }

    //     let dois_to_query: Vec<Doi> = dois
    //         .drain(..CONCURRENCY.min(dois.len()))
    //         .map(|doi| {
    //             queried_dois.insert(doi.clone());
    //             doi
    //         })
    //         .collect();

    //     counter += dois_to_query.len();
    //     let new_lits = query_dois(&dois_to_query).await;
    //     lits.extend(new_lits.clone());

    //     let new_dois = collect_ref_dois(&new_lits);
    //     let new_dois: Vec<Doi> = new_dois
    //         .into_iter()
    //         .unique()
    //         .take(max_counts - counter)
    //         .filter(|doi| !queried_dois.contains(doi))
    //         .collect();

    //     dois.extend(new_dois);
    // }

    // lits
}

fn collect_refs_dois(lits: &Vec<Literature>) -> Vec<Doi> {
    lits.iter()
        .flat_map(|lit| lit.refs.iter().map(|ref_| ref_.doi.clone()))
        .collect()
}
