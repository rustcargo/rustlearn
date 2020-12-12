pub mod from_and_into;
pub mod patter_tips;
pub mod play_iterator;
pub mod str_and_string;

#[cfg(test)]
mod tests {
    #[test]
    fn play_format() {
        assert_eq!(format!("Hello"), "Hello");
        assert_eq!(format!("Hello, {}!", "world"), "Hello, world!");
        assert_eq!(format!("The number is {}", 1), "The number is 1");
        assert_eq!(format!("{:?}", (3, 4)), "(3, 4)");
        // 参数取值
        assert_eq!(format!("{value}", value = 4), "4");
        assert_eq!(format!("{} {}", 1, 2), "1 2");
        // 占位符
        assert_eq!(format!("{:04}", 42), "0042");
        // 设置不同顺序
        assert_eq!(format!("{1} {} {0} {}", 1, 2), "2 1 1 2");
        // 精度控制
        assert_eq!(format!("{:.2}", 123.456), "123.46");
    }

    #[test]
    fn play_sub_str() {
        let mut xyz = "Hello World".to_string();
        let xyz = xyz.as_mut_str(); // 无法直接将字面字符串变成&mut str，只有通过String转化
        assert_eq!(xyz.get(6..), Some("World")); // 索引不在字符边界上
        assert_eq!(xyz.get(20..25), None); // 索引超出范围
    }
}
