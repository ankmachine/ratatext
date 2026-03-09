use reqwest::blocking;

/// Fetches the body of a website as a String
pub fn fetch_data() -> Result<String, Box<dyn std::error::Error>> {
    let response = blocking::get("https://www.rust-lang.org")?; // Send the request
    let body = response.text()?; // Extract the text body
    Ok(body)
}
