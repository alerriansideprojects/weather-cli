fn main() {
    let api_token = std::env::var("API_TOKEN").expect("Expected and API token.");
    let mut arg_iterator = std::env::args();
    arg_iterator.next();
    let args: String = arg_iterator.collect();

    let client = reqwest::blocking::Client::new();

    let response = client
        .get("https://api.waqi.info/search/")
        .query(&[("token", api_token), ("keyword", args)])
        .send()
        .expect("A successful request")
        .json::<serde_json::Value>()
        .expect("Expected body to be json");
    
        dbg!(&response["data"]);
}
