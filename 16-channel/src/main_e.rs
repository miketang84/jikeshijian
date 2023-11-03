use std::time::Duration;
use tokio::task;
use tokio::time;

#[tokio::main]
async fn main() {
    let task_a = task::spawn(async move {
        println!("in task_a");
        time::sleep(Duration::from_secs(3)).await; // 等待3s
        1
    });
    let task_b = task::spawn(async move {
        println!("in task_b");
        2
    });
    let task_c = task::spawn(async move {
        println!("in task_c");
        3
    });

    let (r1, r2, r3) = tokio::join!(task_a, task_b, task_c);

    println!("{}, {}, {}", r1.unwrap(), r2.unwrap(), r3.unwrap());
}
