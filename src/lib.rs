use reqwest::Client;
// This is a library with shared utilities for all solutions
pub async fn fetch_input(day: u32) -> String {
    dotenv::dotenv().ok();
    let url = format!("https://adventofcode.com/2025/day/{}/input", day);
    let session_token = std::env::var("AOC_SESSION").expect("AOC_SESSION not set in .env file");
    
    let response = Client::new()
        .get(&url)
        .header("Cookie", format!("session={}", session_token))
        .send()
        .await
        .expect("Failed to fetch input")
        .text()
        .await
        .expect("Failed to read response");
    response
}
