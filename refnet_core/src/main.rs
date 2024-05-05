use refnet_core::{doi_to_best_literatures, CONCURRENCY};

fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .max_blocking_threads(CONCURRENCY)
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(inner_main());
}

async fn inner_main() {
    let best_lits =
        doi_to_best_literatures("10.1016/j.jempfin.2023.101439".into(), 500, 15, 0.85, 0.1).await;
    dbg!(best_lits.len());
}
