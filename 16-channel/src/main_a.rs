use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let mut db: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let (tx, mut rx) = mpsc::channel::<u32>(100);

    let tx1 = tx.clone();
    let tx2 = tx.clone();

    let task_a = tokio::task::spawn(async move {
        if let Err(_) = tx1.send(50).await {
            println!("receiver dropped");
            return;
        }
    });
    let task_b = tokio::task::spawn(async move {
        if let Err(_) = tx2.send(100).await {
            println!("receiver dropped");
            return;
        }
    });

    let task_c = tokio::task::spawn(async move {
        while let Some(i) = rx.recv().await {
            println!("got = {}", i);
            db[4] = i;
            println!("{:?}", db);
        }
    });
    _ = task_a.await.unwrap();
    _ = task_b.await.unwrap();
    _ = task_c.await.unwrap();
}
