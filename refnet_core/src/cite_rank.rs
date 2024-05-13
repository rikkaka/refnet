use hashbrown::HashMap;

use chrono::{Local, NaiveDate};

use crate::types::{Doi, Literature};

#[derive(Debug)]
struct Fields {
    score: f64,
    init_score: f64,
    reffing_counts: u32,
    reffed: Vec<Doi>,
}
type Data = HashMap<Doi, Fields>;

#[derive(Debug)]
pub struct CiteRankScore {
    pub doi: Doi,
    pub score: f64,
}

pub struct CiteRankBuilder {
    literatures: Vec<Literature>,
    alpha: f64,
    decay_factor: f64,
    current_date: NaiveDate,
}
impl CiteRankBuilder {
    pub fn new(literatures: Vec<Literature>, alpha: f64, decay_factor: f64) -> Self {
        let current_date = Local::now().naive_local().into();
        Self {
            literatures,
            current_date,
            decay_factor,
            alpha,
        }
    }

    pub fn initial_score(&self, date: Option<NaiveDate>) -> f64 {
        let date = date.unwrap_or(NaiveDate::from_ymd_opt(1980, 1, 1).unwrap());
        let age = (self.current_date - date).num_days();
        (-self.decay_factor * age as f64 / 365.).exp()
    }

    pub fn build(self) -> CiteRank {
        let mut data = HashMap::new();

        // 将每个文献插入data
        for lit in &self.literatures {
            let init_score = self.initial_score(lit.date);
            let fields = Fields {
                score: 0.,
                init_score,
                reffing_counts: lit.refs.len() as u32,
                reffed: vec![],
            };
            data.insert(lit.doi.clone(), fields);
        }

        // 计算所有文献的被索引情况
        for lit in &self.literatures {
            for ref_ in &lit.refs {
                match data.get_mut(&ref_.doi) {
                    Some(v) => {
                        v.reffed.push(lit.doi.clone());
                    }
                    None => {
                        let init_score = self.initial_score(ref_.date);
                        let fields = Fields {
                            score: 0.,
                            init_score,
                            reffing_counts: 0,
                            reffed: vec![lit.doi.clone()],
                        };
                        data.insert(ref_.doi.clone(), fields);
                    }
                }
            }
        }

        CiteRank {
            data,
            alpha: self.alpha,
        }
    }
}

#[derive(Debug)]
pub struct CiteRank {
    data: Data,
    alpha: f64,
}

impl CiteRank {
    fn renew_scores(&mut self) {
        let new_scores: Vec<(Doi, f64)> = self
            .data
            .iter()
            .map(|(doi, fields)| {
                let new_score = self.alpha
                    * fields
                        .reffed
                        .iter()
                        .map(|r| {
                            let r = self.data.get(r).unwrap();
                            r.score / r.reffing_counts as f64
                        })
                        .sum::<f64>()
                    + (1. - self.alpha) * fields.init_score;
                (doi.clone(), new_score)
            })
            .collect();

        for (doi, score) in new_scores {
            let fields = self.data.get_mut(&doi).unwrap();
            fields.score = score;
        }
    }

    pub fn renew_iteration(&mut self, iterations: u32) {
        for _ in 0..iterations {
            self.renew_scores()
        }
    }

    pub fn cite_rank_scores(&self) -> Vec<CiteRankScore> {
        let mut scores: Vec<CiteRankScore> = self
            .data
            .iter()
            .map(|(doi, fields)| CiteRankScore {
                doi: doi.clone(),
                score: fields.score,
            })
            .collect();
        scores.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        scores
    }
}
