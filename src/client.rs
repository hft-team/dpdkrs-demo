#![deny(warnings)]
#![warn(rust_2018_idioms)]

use hyper::{body::HttpBody as _, Client};
use tokio::io::{self, AsyncWriteExt as _};

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    dpdk_io::bootstrap();

    let a = dpdk_io::dpdk_agent();

    let b = dpdk_io::dpdk_agent();
    drop(a);
    drop(b);

    pretty_env_logger::init();

    // Some simple CLI args requirements...
    let url = String::from("http://www.baidu.com");
    // HTTPS requires picking a TLS implementation, so give a better
    // warning if the user tries to request an 'https' URL.
    let url = url.parse::<hyper::Uri>().unwrap();
    if url.scheme_str() != Some("http") {
        println!("This example only works with 'http' URLs.");
        return Ok(());
    }

    fetch_url(url).await.expect("");

    loop {}

    // Ok(())
}

#[allow(dead_code)]
async fn fetch_url(url: hyper::Uri) -> Result<()> {
    let client = Client::new();

    let mut res = client.get(url).await?;

    println!("Response: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());

    // Stream the body, writing each chunk to stdout as we get it
    // (instead of buffering and printing at the end).
    while let Some(next) = res.data().await {
        let chunk = next?;
        io::stdout().write_all(&chunk).await?;
    }

    println!("\n\nDone!");

    Ok(())
}
