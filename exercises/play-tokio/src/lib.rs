#[cfg(test)]
mod tests {
    use mini_redis::client;

    #[tokio::test]
    async fn it_works() {
        let mut cli = client::connect("127.0.0.1:6379").await.unwrap();
        cli.set("hello", "world".into()).await.unwrap();
        let result = cli.get("hello").await.unwrap();
        println!("{:?}", result);
    }
}
