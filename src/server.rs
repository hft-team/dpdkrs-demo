// use std::convert::Infallible;
// use std::error::Error;
// use std::net::{IpAddr, Ipv4Addr, SocketAddr};

// use dpdk_io::service::bootstrap;
// use http_body_util::Full;
// use hyper::body::Bytes;
// use hyper::service::service_fn;
// use hyper::{Request, Response};

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
//     env_logger::init();

//     bootstrap();

//     let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 80);

//     let listener = dpdk_io::dpdk_agent().listen(addr).expect("listener");

//     log::info!("build listener success");
//     // let listener = TcpListener::bind(addr).await?;

//     loop {
//         let (stream, _) = listener.accept().await;

//         tokio::task::spawn(async move {
//             if let Err(err) = http1::Builder::new()
//                 .serve_connection(stream, service_fn(hello))
//                 .await
//             {
//                 println!("Error serving connection: {:?}", err);
//             };
//         });
//     }
// }


fn main(){}