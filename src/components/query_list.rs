use reqwest;
use tokio::sync::mpsc;

pub async fn fetch_data(tx: mpsc::UnboundedSender<String>) {
    // Perform the background work
    if let Ok(response) = reqwest::get("https://api.example.com").await {
        if let Ok(text) = response.text().await {
            // Send the result back to the main loop
            let _ = tx.send(text);
        }
    }
}
