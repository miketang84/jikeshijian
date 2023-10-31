use std::env;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8888".to_string());
    // 连接到服务端
    let mut stream = TcpStream::connect(&addr).await?;

    // 写入指令数据，这是一种最简单的协议.
    stream.write_all(b"gettime").await?;

    // 等待tcp server的回复，读取内容
    // 这里用动态数组来存储从服务端返回的内容
    let mut buf: Vec<u8> = Vec::with_capacity(8128);
    // 读取的缓冲区
    let mut resp = [0u8; 2048];
    loop {
        // 尝试一次读，返回读到的字节数
        let n = stream.read(&mut resp).await?;
        // 将读到的字节合并到buf中去
        buf.extend_from_slice(&resp[0..n]);
        if n == 0 {
            // 流断掉了
            panic!("Unexpected EOF");
        } else if buf.len() >= 28 {
            // like: "Tue Oct 31 14:56:27 CST 2023"
            // buf 已经填充了足够的内容
            break;
        } else {
            // buf 中还没有足够多的内容，继续填充...
            continue;
        }
    }

    // 转换并打印返回的信息
    let timeinfo = String::from_utf8(buf)?;
    println!("{}", timeinfo);

    Ok(())
}
