// use mini_redis::{client, Result};

// #[tokio::main]
// pub async fn main() -> Result<()> {
//     // 打开链接到mini-redis的链接
//     let mut client = client::connect("127.0.0.1:6379").await?;

//     // 设置 "hello" 键的值为 "world"
//     client.set("hello", "world".into()).await?;

//     // 获取"hello"的值
//     let result = client.get("hello").await?;

//     println!("got value from the server; result={:?}", result);

//     Ok(())
// }
use tokio::fs::File;
use tokio::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut reader: &[u8] = b"hello";
    let mut file = File::create("foo.txt").await?;

    io::copy(&mut reader, &mut file).await?;
    Ok(())
}
