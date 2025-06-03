use gloo::storage::{LocalStorage, Storage};

mod error;

#[derive(serde::Deserialize)]
pub struct Session {
    pub token: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct Challenge {
    pub id: String,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Score {
    pub completed: bool,
    pub details: String,
    pub error: bool,
    pub rank: i32,
    pub tips: u32,
    pub tries: u32,
    pub word: String,
}

impl Score {
    pub fn new() -> Self {
        Self {
            completed: false,
            details: String::new(),
            error: false,
            rank: -1,
            tips: 0,
            tries: 0,
            word: String::new(),
        }
    }
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

    pub async fn initialize_token(&mut self) {
        let token = LocalStorage::get::<String>("token");

        if let Ok(token) = token {
            self.token = Some(token);
        } else {
            let session = Contextno::initialize_session().await.unwrap();
            LocalStorage::set::<String>("token", session.token.clone());

            self.token = Some(session.token);
        }
    }

    pub async fn initialize_session() -> Result<Session, reqwest::Error> {
        reqwest::get(format!("{API_URL}initialize_session"))
            .await?
            .error_for_status()?
            .json::<Session>()
            .await
    }

    pub async fn get_random_challenge(&self) -> Result<Challenge, reqwest::Error> {
        reqwest::get(format!(
            "{API_URL}get_random_challenge?user_id={}",
            self.token.as_ref().unwrap()
        ))
        .await?
        .error_for_status()?
        .json::<Challenge>()
        .await
    }

    pub async fn get_score(
        &self,
        challenge_id: String,
        word: String,
    ) -> Result<Score, reqwest::Error> {
        reqwest::get(format!("{API_URL}get_score?challenge_id={challenge_id}&user_id={}&word={word}&challenge_type=random", self.token.as_ref().unwrap()))
            .await?
            .error_for_status()?
            .json::<Score>()
            .await
    }
}
