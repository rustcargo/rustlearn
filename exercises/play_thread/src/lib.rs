use std::thread;

pub fn sample_map_reduce(data: &'static str) -> u32 {
    // 创建一个向量，用于储存将要创建的子线程
    let mut children = vec![];
    // 把数据分段，每段将会单独计算
    // 每段都是完整数据的一个引用（&str
    let chunked_data = data.split_whitespace();

    for data_segment in chunked_data.into_iter() {
        children.push(thread::spawn(move || -> u32 {
            data_segment.chars().map(|c| c.to_digit(10).expect("should be a digit")).sum::<u32>()
        }))
    }
    // 把每个线程产生的中间结果收入一个新的向量中
    let mut intermediate_sums = vec![];

    for child in children {
        intermediate_sums.push(child.join().unwrap_or(0))
    }

    intermediate_sums.iter().sum()
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::fs::File;
    use std::io::{Write, Read};
    use std::process::Command;

    #[test]
    fn play_sample_map_reduce() {
        let data = "123 456";
        assert_eq!(super::sample_map_reduce(data), 21);
    }

    ///注意 Path 在内部并不是用 UTF-8 字符串表示的，而是存储为若干字节（Vec<u8>）的 vector。
    /// 因此，将 Path 转化成 &str 并非零开销的（free），且可能失败（因此它 返回一个 Option）。
    #[test]
    fn play_path() {
        let path = Path::new(".");
        assert_eq!(path.display().to_string(), ".");

        let new_path = path.join("a").join("b");
        assert_eq!(new_path.to_str().unwrap(), "./a/b")
    }

    #[test]
    fn play_basic_file() {
        let path = Path::new("test.txt");
        // 以只写模式打开文件，返回 `io::Result<File>`
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}", path.display()),
            Ok(file) => file,
        };

        file.write_all(b"123456");
        let mut s= String::new();

        // 以只读方式打开路径，返回 `io::Result<File>`
        let mut file = match File::open(&path) {
            // `io::Error` 的 `description` 方法返回一个描述错误的字符串。
            Err(why) => panic!("couldn't open {}", path.display()),
            Ok(file) => file,
        };

        file.read_to_string(&mut s);
        assert_eq!(s, "123456")
    }

    #[test]
    fn play_process_command() {
        let output = Command::new("rustc").arg("--version").output().unwrap();
        assert!(output.status.success());
        let version = String::from_utf8_lossy(&output.stdout).to_lowercase();
        assert_eq!(version.get(0..10).unwrap(), "rustc 1.39")
    }
}
