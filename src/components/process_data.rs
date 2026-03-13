use color_eyre::{Result, eyre};
use std::time::Duration;
use tokio::time::sleep;

pub struct DataProcessor;

impl DataProcessor {
    pub fn new() -> Self {
        Self
    }

    pub async fn fetch_list(&self) -> Result<Vec<String>> {
        // simulate network delay
        sleep(Duration::from_millis(500)).await;

        let response = self.make_http_request().await?;
        let list_data = self.process_response(response).await?;
        Ok(list_data)
    }

    async fn make_http_request(self) -> Result<String> {
        let client = reqwest::Client::new();
        let res = client
            .get("https://httpbin.org/json")
            .send()
            .await
            .map_err(|r| color_eyre::eyre::eyre!("http request failed: {}", e))?;
        let body = res
            .text()
            .await
            .map_err(|e| color_eyre::eyre::eyre!("Failed to read body : {}", e))?;
        Ok(body)
    }

    async fn process_response(&self, _response: String) -> Result<Vec<String>> {
        sleep(Duration::from_millis(200)).await;
        let mut result = Vec::new();
        result.extend(vec![
            "ALPHA".to_string(),
            "GAMMA".to_string(),
            "BETA".to_string(),
            "DELTA".to_string(),
        ]);
        Ok(result)
    }
}
impl Default for DataProcessor {
    fn default() -> Self {
        Self::new()
    }
}
