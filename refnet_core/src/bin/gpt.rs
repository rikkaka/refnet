use std::io::{stdout, Write};

use futures::StreamExt;
use refnet_core::{doi_to_best_literatures, gen_review};

#[tokio::main]
async fn main() {
    let lits =
        doi_to_best_literatures("10.1038/s41586-024-07336-w".into(), 500, 15, 0.9, 0.08).await;
    let dois: Vec<_> = lits.iter().map(|lit| lit.doi.clone()).collect();
    let mut stream = gen_review(&dois).await;

    let mut lock = stdout().lock();
    while let Some(result) = stream.next().await {
        match result {
            Ok(response) => {
                response.choices.iter().for_each(|chat_choice| {
                    if let Some(ref content) = chat_choice.delta.content {
                        write!(lock, "{}", content).unwrap();
                    }
                });
            }
            Err(err) => {
                writeln!(lock, "error: {err}").unwrap();
            }
        }
        stdout().flush().unwrap();
    }
}
