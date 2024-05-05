mod controllers;
mod utils;

use refnet_core::CONCURRENCY;

use salvo::prelude::*;

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
    std::env::set_var("RUST_LOG", "debug");
    
    let local_addr = match std::env::var("LOCAL_ADDR") {
        Ok(addr) => addr.leak(),
        Err(_) => {
            println!("LOCAL_ADDR unset. Default to 127.0.0.1:3030");
            "127.0.0.1:3030"
        }
    };

    tracing_subscriber::fmt::init();

    let acceptor = TcpListener::new(local_addr).bind().await;

    let router = Router::new()
        .push(Router::with_path("refnet").push(Router::with_path("doi").get(controllers::from_doi)))
        .push(
            Router::new()
                .path("<**refnet>")
                .goal(Proxy::use_hyper_client("http://localhost:8080/")),
        );

    Server::new(acceptor).serve(router).await;
}
