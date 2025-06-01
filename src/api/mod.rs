use tracing::instrument;

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

const API_URL: &str = "https://апи.контекстно.рф/";
pub const USER_ID: &str = "cf7274b8-9128-4dd0-9282-19c64d727ec7";

#[derive(Clone)]
pub struct Contextno {}

impl Contextno {
    pub async fn get_random_challenge() -> Result<Challenge, reqwest::Error> {
        reqwest::get(format!("{API_URL}get_random_challenge?user_id={USER_ID}"))
            .await?
            .error_for_status()?
            .json::<Challenge>()
            .await
    }

    #[instrument(err, skip_all, ret)]
    pub async fn get_score(challenge_id: String, word: String) -> Result<Score, reqwest::Error> {
        reqwest::get(format!("{API_URL}get_score?challenge_id={challenge_id}&user_id={USER_ID}&word={word}&challenge_type=random"))
            .await?
            .error_for_status()?
            .json::<Score>()
            .await
    }
}
