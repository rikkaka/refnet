use anyhow::Result;
use lazy_static::lazy_static;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

use crate::types::{Literature, Ref};

lazy_static! {
    static ref POOL: Pool<Sqlite> = SqlitePoolOptions::new()
        .max_connections(50)
        .connect_lazy("sqlite://refdata.sqlite")
        .unwrap();
}

pub async fn insert_literature(literature: Literature) -> Result<()> {
    let refs = serde_json::to_string(&literature.refs)?;
    sqlx::query!(
        r#"
        INSERT INTO literatures (doi, title, author, date_, abstract_, refs)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
        literature.doi,
        literature.title,
        literature.author,
        literature.date,
        literature.abstract_,
        refs
    )
    .execute(&*POOL)
    .await?;

    Ok(())
}

pub async fn get_literature(doi: &str) -> Option<Literature> {
    let row = sqlx::query!(
        r#"
        SELECT * FROM literatures WHERE doi = ?
        "#,
        doi
    )
    .fetch_one(&*POOL)
    .await
    .ok()?;

    let refs: Option<Vec<Ref>> = row.refs.map(|refs| serde_json::from_str(&refs).unwrap());
    let date = row.date_;

    Some(Literature {
        doi: row.doi,
        title: row.title.unwrap_or_default(),
        author: row.author.unwrap_or_default(),
        date,
        abstract_: row.abstract_,
        refs: refs.unwrap_or_default(),
    })
}
