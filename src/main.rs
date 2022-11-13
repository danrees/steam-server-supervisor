use tokio::sync::mpsc;
#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<String>(10);

    let handle = tokio::spawn(async move {
        //let rx_clone = rx.clone();
        while let Some(msg) = rx.recv().await {
            println!("Got: {}", msg);
        }
    });

    let sender_thread = tokio::spawn(async move {
        let sender = tx.clone();
        let mut timer = tokio::time::interval(chrono::Duration::seconds(5).to_std().unwrap());
        loop {
            timer.tick().await;
            match sender.send(String::from("tick")).await {
                Ok(_) => println!("o -> ticked"),
                Err(e) => println!("e -> {}", e),
            };
        }
    });
    //join!(handle, sender_thread).await;
    handle.await.unwrap();
    sender_thread.await.unwrap();
}
