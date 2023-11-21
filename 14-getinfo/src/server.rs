use std::env;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8888".to_string());
    println!("Listening on: {}", addr);
    let listener = TcpListener::bind(&addr).await?;

    // 注意这里是一个无条件循环，表明始终处于服务状态
    loop {
        // 等待客户端请求连上来
        let (mut socket, _) = listener.accept().await?;

        // 来一个客户端连接，创建一个对应的新任务
        tokio::spawn(async move {
            // 分配一个缓冲存
            let mut buf = [0; 1024];
            let mut offset = 0;
            // 循环读，因为不能确保一次能从网络线路上读完数据
            loop {
                // 读操作，返回的n表示读了多少个字节
                // 正常情况下，读到数据才会返回，如果没有读到，就会等待
                let n = socket
                    .read(&mut buf[offset..])
                    .await
                    .expect("failed to read data from socket");

                // n返回0的情况，是碰到了EOF，表明远端的写操作已断开，这个一定要判断
                if n == 0 {
                    // 碰到了EOF就直接返回结束此任务，因为后面的操作没了意义
                    return;
                }

                println!("offset: {offset}, n: {n}");
                let end = offset + n;
                // 转换指令为字符串
                if let Ok(directive) = std::str::from_utf8(&buf[..end]) {
                    println!("{directive}");
                    // 执行指令对应的工作
                    let output = process(directive).await;
                    println!("{output}");
                    // 向客户端返回处理结果
                    socket
                        .write_all(&output.as_bytes())
                        .await
                        .expect("failed to write data to socket");
                } else {
                    // 判断是否转换失败，如果失败，就有可能是网络上的数据还没读完
                    // 要继续loop读下一波数据
                    offset = end;
                }
            }
        });
    }
}

async fn process(directive: &str) -> String {
    if directive == "gettime" {
        // 这里我们用了unwrap()是因为我们一般确信执行date命令不会失败
        // 更可靠的作法是对返回的Result作处理
        let output = Command::new("date").output().await.unwrap();
        String::from_utf8(output.stdout).unwrap()
    } else {
        // 如果是其它指令，我们目前返回 无效指令
        "invalid command".to_owned()
    }
}
