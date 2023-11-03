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

    let mut tasks = Vec::with_capacity(3);
    tasks.push(task_a);
    tasks.push(task_b);
    tasks.push(task_c);

    let mut outputs = Vec::with_capacity(tasks.len());
    for task in tasks {
        println!("iterate task result..");
        // 在这里依次等待任务完成
        outputs.push(task.await.unwrap());
    }
    println!("{:?}", outputs);
}
