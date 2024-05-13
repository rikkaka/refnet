use chrono::Datelike;
use cite_rank::CiteRankScore;
use serde::{Deserialize, Serialize};

mod cite_rank;
mod data_deal;
mod gpt;
mod types;

pub use async_openai::types::ChatCompletionResponseStream;
pub use gpt::gen_review;

pub const CONCURRENCY: usize = 40;
const YEAR: i32 = 2024;

#[derive(Debug, Serialize, Deserialize)]
pub struct LiteratureRet {
    pub doi: String,
    pub title: String,
    pub author: String,
    pub year: Option<i32>,
    pub refs: Vec<String>,
    pub score: f64,
}

pub async fn doi_to_best_literatures(
    doi: String,
    extend_num: usize,
    best_num: usize,
    alpha: f64,
    decay_factor: f64,
) -> Vec<LiteratureRet> {
    let lits = data_deal::extend(doi, extend_num, CONCURRENCY).await;

    let mut cite_rank = cite_rank::CiteRankBuilder::new(lits, alpha, decay_factor).build();
    cite_rank.renew_iteration(30);
    let best_lit_scores = cite_rank
        .cite_rank_scores()
        .into_iter()
        .take(best_num)
        .collect();
    scores_to_lit_rets(best_lit_scores).await
}

async fn scores_to_lit_rets(lit_socres: Vec<CiteRankScore>) -> Vec<LiteratureRet> {
    let dois = lit_socres
        .iter()
        .map(|lit| lit.doi.clone())
        .collect::<Vec<_>>();
    let lits = data_deal::query_dois(&dois).await;
    lits.into_iter()
        .zip(lit_socres)
        .map(|(mut lit, score)| {
            lit.refs = lit
                .refs
                .into_iter()
                .filter(|ref_| dois.contains(&ref_.doi))
                .collect();
            LiteratureRet {
                doi: lit.doi,
                title: lit.title,
                author: lit.author,
                year: lit.date.map(|date| date.year()),
                refs: lit.refs.iter().map(|ref_| ref_.doi.clone()).collect(),
                score: score.score,
            }
        })
        .collect()
}
