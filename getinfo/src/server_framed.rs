use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use std::env;
use tokio::net::TcpListener;
use tokio::process::Command;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

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
        let (stream, _) = listener.accept().await?;
        // 包裹成一个Frame stream
        let mut framed_stream = Framed::new(stream, LengthDelimitedCodec::new());

        // 创建子task执行任务
        tokio::spawn(async move {
            // 等待读取一个一个msg，如果返回None，会退出这个循环
            while let Some(msg) = framed_stream.next().await {
                match msg {
                    Ok(msg) => {
                        // 解析指令，执行任务
                        let directive = String::from_utf8(msg.to_vec())
                            .expect("error when converting to string directive.");
                        println!("{directive}");
                        let output = process(&directive).await;
                        println!("{output}");

                        // 返回执行结果
                        _ = framed_stream.send(Bytes::from(output)).await;
                    }
                    Err(e) => {
                        println!("{e:?}");
                    }
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
