use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() == 1 { "" } else { &args[1] };
    let query_input = serde_json::to_vec(&format!(r#"{}"#, input))?;

    let query = reqwest::Client::new()
        .post("http://127.0.0.1:5000/lemma")
        .header("Content-Type", "application/json")
        .body(query_input)
        .send()
        .await?;

    let response: Vec<String> = query.json().await?;

    println!("{:?}", response);

    Ok(())
}
