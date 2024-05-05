use hashbrown::HashSet;

pub mod cite_rank;
pub mod data_deal;
pub mod types;

pub const CONCURRENCY: usize = 30;
const YEAR: i32 = 2024;

pub async fn doi_to_best_literatures(
    doi: String,
    extext_num: usize,
    best_num: usize,
    alpha: f64,
    decay_factor: f64,
) -> Vec<types::BriefLiterature> {
    let lits = data_deal::extend(doi, extext_num).await;

    let mut cite_rank = cite_rank::CiteRankBuilder::new(lits, alpha, decay_factor).build();
    cite_rank.renew_iteration(30);
    let best_lits = cite_rank.best_literatures(best_num).await;

    let best_dois: HashSet<String> = best_lits.iter().map(|lit| lit.doi.clone()).collect();

    best_lits
        .into_iter()
        .map(|mut lit| {
            lit.refs = lit
                .refs
                .iter()
                // 仅保留在best_dois中存在的引用
                .filter(|ref_| best_dois.contains(&ref_.doi as &str))
                .cloned()
                .collect();
            lit.into()
        })
        .collect()
}
