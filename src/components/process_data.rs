use color_eyre::Result;
use std::time::Duration;
use tokio::sync::mpsc::UnboundedSender;
use tokio::time::{interval, sleep};

#[derive(Clone, Copy)]
pub struct DataProcessor;

impl DataProcessor {
    pub fn new() -> Self {
        Self
    }

    pub async fn fetch_initial_list(&self) -> Result<Vec<String>> {
        // simulate network delay
        sleep(Duration::from_millis(500)).await;

        let _response = self.make_http_request().await?;
        let list_data = self.get_initial_batch().await?;
        Ok(list_data)
    }

    pub fn start_streaming_list(sender: UnboundedSender<crate::action::Action>) {
        tokio::spawn(async move {
            let all_item = Self::get_all_list_static().await;
            let initial_count = 8;
            let streaming_list: Vec<String> = all_item.into_iter().skip(initial_count).collect();
            let mut inter = interval(Duration::from_millis(2000)); // delay new item by 2 sec
            for item in streaming_list {
                inter.tick().await;
                let _ = sender.send(crate::action::Action::NewItemLoaded(item));
            }
            let _ = sender.send(crate::action::Action::StreamingComplete);
        });
    }

    async fn make_http_request(self) -> Result<String> {
        let client = reqwest::Client::new();
        let res = client
            .get("https://httpbin.org/json")
            .send()
            .await
            .map_err(|e| color_eyre::eyre::eyre!("http request failed: {}", e))?;
        let body = res
            .text()
            .await
            .map_err(|e| color_eyre::eyre::eyre!("Failed to read body : {}", e))?;
        Ok(body)
    }

    async fn get_initial_batch(&self) -> Result<Vec<String>> {
        sleep(Duration::from_millis(200)).await;
        let result = vec![
            "ALPHA".to_string(),
            "GAMMA".to_string(),
            "BETA".to_string(),
            "DELTA".to_string(),
            "Some".to_string(),
            "cherry".to_string(),
            "workflow".to_string(),
            "tiny".to_string(),
        ];
        Ok(result)
    }

    async fn get_all_list_static() -> Vec<String> {
        vec![
            "ALPHA".to_string(),
            "GAMMA".to_string(),
            "BETA".to_string(),
            "DELTA".to_string(),
            "Some".to_string(),
            "cherry".to_string(),
            "workflow".to_string(),
            "tiny".to_string(),
            "yaml".to_string(),
            "check".to_string(),
            "test".to_string(),
            "machine".to_string(),
            "workflow".to_string(),
            "repo".to_string(),
            "system".to_string(),
            "age".to_string(),
            "empire".to_string(),
            "hera".to_string(),
            "lirery".to_string(),
            "seba".to_string(),
            "viper".to_string(),
            "nilli".to_string(),
            "game".to_string(),
            "comment".to_string(),
            "like".to_string(),
            "iron".to_string(),
            "cast".to_string(),
            "boat".to_string(),
            "fish".to_string(),
            "distance".to_string(),
            "questions".to_string(),
            "beautiful".to_string(),
        ]
    }

    pub async fn fetch_initial_list_safe(&self) -> Vec<String> {
        match self.fetch_initial_list().await {
            Ok(list_data) => list_data,
            Err(e) => {
                eprintln!("Failed to fetch: {}", e);
                vec!["Bad Network".to_string()]
            }
        }
    }

    pub fn start_streaming_list_safe(sender: UnboundedSender<crate::action::Action>) {
        Self::start_streaming_list(sender);
    }
}
impl Default for DataProcessor {
    fn default() -> Self {
        Self::new()
    }
}
