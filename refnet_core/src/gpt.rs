use chrono::Datelike;
use hashbrown::HashMap;
use serde::Serialize;

use async_openai::{
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        ChatCompletionResponseStream, CreateChatCompletionRequestArgs,
    },
    Client,
};

use crate::{
    data_deal::query_dois,
    types::{Doi, Literature},
};

async fn gen(system: &str, user: &str) -> ChatCompletionResponseStream {
    let client = Client::new();

    let messages = [
        ChatCompletionRequestSystemMessageArgs::default()
            .content(system)
            .build()
            .unwrap()
            .into(),
        ChatCompletionRequestUserMessageArgs::default()
            .content(user)
            .build()
            .unwrap()
            .into(),
    ];

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-3.5-turbo")
        .temperature(0.1)
        .messages(messages)
        .build()
        .unwrap();

    client.chat().create_stream(request).await.unwrap()
}

#[derive(Serialize)]
struct LiteratureGpt {
    number: u32,
    title: String,
    author: String,
    year: i32,
    #[serde(rename = "abstract")]
    abstract_: Option<String>,
    references: Vec<u32>,
}

fn lits_to_litgpts(lits: Vec<Literature>) -> Vec<LiteratureGpt> {
    let doi_number_map: HashMap<_, _> = lits
        .iter()
        .enumerate()
        .map(|(i, lit)| (lit.doi.clone(), i as u32))
        .collect();
    lits.into_iter()
        .map(|lit| {
            let references = lit
                .refs
                .iter()
                .filter_map(|ref_| doi_number_map.get(&ref_.doi).cloned())
                .collect();
            LiteratureGpt {
                number: doi_number_map[&lit.doi],
                title: lit.title,
                author: lit.author,
                year: lit.date.unwrap().year(),
                abstract_: lit.abstract_,
                references,
            }
        })
        .collect()
}

pub async fn gen_review(dois: &[Doi]) -> ChatCompletionResponseStream {
    let lits = query_dois(dois).await;
    let system = "You will be provided with an array of objects representing scholarly articles in JSON format. Each object contains the following fields:
    - number (integer): The unique identifier for each article.
    - title (string): The title of the article.
    - author (string): The name of the article's author.
    - year (integer): The publication year of the article.
    - abstract (string | null): The abstract of the article, if available. If the abstract is null, you should utilize the information contained in the title and your own knowledge to infer the context and content of the article.
    - references ([integer]): An array of numbers corresponding to the articles cited by this one.
    
    Based on the information provided, your task is to write a literature review in the style of Chinese academic papers. The output should be coherent, well-structured, and fully in Chinese. When organizing the review, consider the chronological order of the articles based on their publication year to highlight the development and evolution of the research topic. When the abstract is missing, use the title and relevant contextual knowledge to bridge gaps and ensure a comprehensive understanding of each article's contributions. The review should integrate the provided data, discuss the significance and impact of the cited works, and link the articles together in a meaningful academic discourse. Don't mention the title of articles in the review. Be sure to follow academic integrity and appropriately acknowledge the contributions of cited works.
IMPORTANT: Follow the style of Chinese academic papers!
IMPORTANT: Use as many as provided articles as possible, ensuring a comprehensive understanding over the articles! The longer the literature review, the better!";
    let user = serde_json::to_string(&lits_to_litgpts(lits)).unwrap();

    gen(system, &user).await
}
