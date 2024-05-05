use std::time::Instant;

use crossref::Crossref;

fn main() {
    let client = Crossref::builder()
        .proxy("http://127.0.0.1:7890")
        .polite("dsywh123@gmail.com")
        .build()
        .unwrap();

    let now = Instant::now();
    let _ = client.work("10.1016/j.jempfin.2023.101439");
    dbg!(now.elapsed().as_secs_f64());

    // println!("{work:?}");
}
