use std::time::Duration;
use tokio::sync::{mpsc, oneshot};
use tokio::task;
use tokio::time;

#[tokio::main]
async fn main() {
    let mut db: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let (tx, mut rx) = mpsc::channel::<(u32, oneshot::Sender<bool>)>(100);

    let tx1 = tx.clone();
    let tx2 = tx.clone();

    let task_a = task::spawn(async move {
        time::sleep(Duration::from_secs(3)).await;
        let (resp_tx, resp_rx) = oneshot::channel();
        if let Err(_) = tx1.send((50, resp_tx)).await {
            println!("receiver dropped");
            return;
        }
        if let Ok(ret) = resp_rx.await {
            if ret {
                println!("task_a finished with success.");
            } else {
                println!("task_a finished with failure.");
            }
        } else {
            println!("oneshot sender dropped");
            return;
        }
    });
    let task_b = task::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        if let Err(_) = tx2.send((100, resp_tx)).await {
            println!("receiver dropped");
            return;
        }
        if let Ok(ret) = resp_rx.await {
            if ret {
                println!("task_b finished with success.");
            } else {
                println!("task_b finished with failure.");
            }
        } else {
            println!("oneshot sender dropped");
            return;
        }
    });

    let task_c = task::spawn(async move {
        while let Some((i, resp_tx)) = rx.recv().await {
            println!("got = {}", i);
            db[4] = i;
            println!("{:?}", db);
            resp_tx.send(true).unwrap();
        }
    });

    _ = task_a.await.unwrap();
    _ = task_b.await.unwrap();
    _ = task_c.await.unwrap();
}
