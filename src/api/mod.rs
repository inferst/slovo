use serde::{Deserialize, Serialize};

mod error;
use crate::api::error::Error;

#[derive(Deserialize)]
pub struct Session {
    pub token: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Challenge {
    pub id: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub struct Score {
    pub completed: bool,
    pub details: String,
    pub error: bool,
    pub rank: i32,
    pub tips: u32,
    pub tries: u32,
    pub word: String,
}

const API_URL: &str = "https://апи.контекстно.рф/";

#[derive(Clone)]
pub struct Contextno {
    token: Option<String>,
}

impl Contextno {
    pub fn new() -> Self {
        Contextno { token: None }
    }

    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }

    pub fn get_token(&self) -> Result<&str, Error> {
        if let Some(token) = &self.token {
            Ok(token)
        } else {
            Err(error::Error::EmptyToken)
        }
    }

    pub async fn initialize_session() -> Result<Session, Error> {
        let result = reqwest::get(format!("{API_URL}initialize_session"))
            .await?
            .error_for_status()?
            .json::<Session>()
            .await?;

        Ok(result)
    }

    pub async fn get_random_challenge(&self) -> Result<Challenge, Error> {
        let token = self.get_token()?;
        let result = reqwest::get(format!("{API_URL}get_random_challenge?user_id={}", token))
            .await?
            .error_for_status()?
            .json::<Challenge>()
            .await?;

        Ok(result)
    }

    pub async fn get_score(&self, challenge_id: String, word: String) -> Result<Score, Error> {
        let token = self.get_token()?;
        let result = reqwest::get(format!("{API_URL}get_score?challenge_id={challenge_id}&user_id={}&word={word}&challenge_type=random", token))
            .await?
            .error_for_status()?
            .json::<Score>()
            .await?;

        Ok(result)
    }
}
