use serde::{Deserialize, Serialize};

mod error;
use crate::api::error::Error;

#[derive(Deserialize, Debug, Clone)]
pub struct Challenge {
    pub id: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub struct Score {
    pub error: Option<String>,
    pub distance: Option<i32>,
    pub word: String,
}

const API_URL: &str = "https://апи.контекстно.рф/";

#[derive(Clone)]
pub struct Contextno {}

impl Contextno {
    pub async fn get_random_challenge() -> Result<Challenge, Error> {
        let result = reqwest::get(format!("{API_URL}random-challenge"))
            .await?
            .error_for_status()?
            .json::<Challenge>()
            .await?;

        Ok(result)
    }

    pub async fn get_score(challenge_id: String, word: String) -> Result<Score, Error> {
        let result = reqwest::get(format!(
            "{API_URL}score?challenge_id={challenge_id}&word={word}&challenge_type=random"
        ))
        .await?
        .error_for_status()?
        .json::<Score>()
        .await?;

        Ok(result)
    }
}
