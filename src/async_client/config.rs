use reqwest::{Client, Error, StatusCode};
use serde_json::Value;
use std::{error, io::ErrorKind, time::Duration};
use tokio::time::sleep;

const MAX_RETRIES: u32 = 5;
const INITIAL_BACKOFF: Duration = Duration::from_secs(1);

#[derive(Debug, Clone)]
pub struct SanityConfig {
    project_id: String,
    access_token: String,
    data_set: String,
    url: String,
    pub query: Query,
}

#[derive(Debug, Clone)]
pub struct Query {
    base_url: String,
    pub query: Option<String>,
}

pub fn create(project_id: &str, data_set: &str, token: &str, use_prod: bool) -> SanityConfig {
    SanityConfig {
        project_id: project_id.to_string(),
        access_token: token.to_string(),
        data_set: data_set.to_string(),
        url: get_url(project_id, data_set),
        query: Query {
            base_url: if use_prod {
                format!("{} {}", project_id, data_set)
            } else {
                format!("{} {}", project_id, data_set)
            },
            query: None,
        },
    }
}

pub fn get_url(project_id: &str, data_set: &str) -> String {
    format!(
        "https://{}.api.sanity.io/v1/data/query/{}",
        project_id, data_set
    )
}

impl Query {
    pub async fn execute(&self) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{} {}", &self.base_url, &self.query.as_ref().unwrap());
        let res: _ = reqwest::get(&url).await?.text().await?;
        let data: Value = serde_json::from_str(&res)?;
        Ok(data)
    }
}

impl SanityConfig {
    pub async fn build_url(&mut self, query: Option<&str>) -> String {
        match query {
            Some(query) => format!("{}?query={}", self.query.base_url, query),
            None => format!(
                "{}?query={}",
                self.query.base_url,
                self.query.query.as_ref().unwrap()
            ),
        }
    }

    pub async fn get(&mut self, query: &str) -> Result<reqwest::Response, reqwest::Error> {
        let client: Client = reqwest::Client::new();
        let url = self.build_url(Some(query)).await;

        // TODO: Add support for retries
        let res = client
            .get(&url)
            .bearer_auth(&self.access_token)
            .send()
            .await?;

        Ok(res)
    }
}