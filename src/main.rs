use std::env;

use steam_server_supervisor::{server::server_service_server::ServerServiceServer, Service};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let (tx_i, mut rx_i) = mpsc::channel(4);
    //let (tx_r, mut rx_r) = mpsc::channel(4);

    let steam_cmd = env::var("STEAM_CMD")?;
    let base_dir = env::var("BASE_DIR")?;
    let addr = env::var("GRPC_ADDR").unwrap_or(String::from("[::1]:50051"));

    let addr = addr.parse()?;
    let my_service = Service::new(&steam_cmd, &base_dir);

    Server::builder()
        .add_service(ServerServiceServer::new(my_service))
        .serve(addr)
        .await?;

    // let install_thread = tokio::spawn(async move {
    //     while let Some(svr) = rx_i.recv().await {
    //         print!("Received install for: {:?}", svr)
    //     }
    // });

    // let run_thread = tokio::spawn(async move {
    //     while let Some(svr) = rx_r.recv().await {
    //         println!("Received run for: {:?}", svr)
    //     }
    // });

    // install_thread.await?;
    // run_thread.await?;

    // Some sample code from an initial test
    // let (tx, mut rx) = mpsc::channel::<String>(10);

    // let handle = tokio::spawn(async move {
    //     //let rx_clone = rx.clone();
    //     while let Some(msg) = rx.recv().await {
    //         println!("Got: {}", msg);
    //     }
    // });

    // let sender_thread = tokio::spawn(async move {
    //     let sender = tx.clone();
    //     let mut timer = tokio::time::interval(chrono::Duration::seconds(5).to_std().unwrap());
    //     loop {
    //         timer.tick().await;
    //         match sender.send(String::from("tick")).await {
    //             Ok(_) => println!("o -> ticked"),
    //             Err(e) => println!("e -> {}", e),
    //         };
    //     }
    // });
    // //join!(handle, sender_thread).await;
    // handle.await.unwrap();
    // sender_thread.await.unwrap();

    Ok(())
}
