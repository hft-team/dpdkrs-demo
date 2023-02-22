#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dpdk_io::bootstrap();
    let echo_json: serde_json::Value = reqwest::Client::new()
        .get("http://172.31.10.131/ping")
        .send()
        .await
        .expect("send")
        .json()
        .await?;
    println!("{:#?}", echo_json);
    Ok(())
}
