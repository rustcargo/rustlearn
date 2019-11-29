mod simple_future;
mod timer_future;

async fn hello() {
    println!("hello")
}

async fn a(a: i32, b: i32) -> i32 {
    a + b
}

async fn b(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;
    use futures::future;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn play_hello () {
        let future = hello();
        //执行async函数不会导致函数执行，需要指定block_on才会启动函数
        eprintln!("no run,no print");
        block_on(future);
    }

    #[test]
    fn play_block () {
        let r = block_on(async {
            eprintln!("{}", a(1, 2).await);
            eprintln!("{}", b(1, 2).await);

            async fn foo(i: u32) -> u32 {
                i * 10
            }

            let futures = vec![foo(1), foo(2), foo(3)];
            let all_finished = future::join_all(futures).await;
            eprintln!("{:?}", all_finished);

            "block_on result"
        });
        eprintln!("{}", r)
    }
}
