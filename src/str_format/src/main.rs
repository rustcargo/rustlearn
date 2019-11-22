
mod use_string;

use use_string::*;

fn main() {

    assert_eq!(format!("Hello"), "Hello");
    assert_eq!(format!("Hello, {}!", "world"), "Hello, world!");
    assert_eq!(format!("The number is {}", 1), "The number is 1");
    assert_eq!(format!("{:?}", (3, 4)), "(3, 4)");
    // 参数取值
    assert_eq!(format!("{value}", value=4), "4");
    assert_eq!(format!("{} {}", 1, 2), "1 2");
    // 占位符
    assert_eq!(format!("{:04}", 42), "0042");
    // 设置不同顺序
    assert_eq!(format!("{1} {} {0} {}", 1, 2), "2 1 1 2");
    // 精度控制
    assert_eq!(format!("{:.2}", 123.456), "123.46");

    get_string();

}
