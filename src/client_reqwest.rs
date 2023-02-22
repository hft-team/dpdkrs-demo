#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dpdk_io::bootstrap();
    time_analyse().await
}

async fn time_analyse() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let mut time_cost = vec![];
    for _ in 0..100 {
        // tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        // println!("start to curl");
        let start = std::time::Instant::now();
        let _json_body = client.get("http://172.31.10.131/ping").send().await?;
        // .expect("send")
        // .json()
        // .await?;
        // println!("{:#?}", json_body);

        let cost = start.elapsed();
        time_cost.push(cost);
    }

    println!("{:#?}", time_cost);
    let mut sum = std::time::Duration::ZERO;
    for (i, c) in time_cost.iter().enumerate() {
        if i == 0 {
            continue;
        }
        sum += *c;
    }

    println!("ave cost = {:?}", sum / 99);

    Ok(())
}
