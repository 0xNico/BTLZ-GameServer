#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let server_url = "http://127.0.0.1:3000"; // adjust to match server env.

    // Combined tasks for creating and joining battles simultaneously
    let start = std::time::Instant::now();
    let tasks: Vec<_> = (0..1000).flat_map(|_| {
        let create_client = client.clone();
        let join_client = client.clone();
        vec![
            tokio::spawn(async move {
                create_client.get(format!("{}/create_battle", server_url))
                    .send().await.expect("Failed to create battle")
                    .json::<serde_json::Value>().await.expect("Failed to parse create battle response")
            }),
            tokio::spawn(async move {
                join_client.get(format!("{}/join_battle", server_url))
                    .send().await.expect("Failed to join battle")
                    .json::<serde_json::Value>().await.expect("Failed to parse join battle response")
            })
        ]
    }).collect();

    // Await all tasks to complete
    for task in tasks {
        let _ = task.await;
    }

    println!("Total time for creating and joining 1,000 battles: {:?}", start.elapsed());
}
