
pub fn get_string() {
    let mut xyz = "Hello World".to_string();
    let xyz = xyz.as_mut_str();// 无法直接将字面字符串变成&mut str，只有通过String转化
    assert_eq!(xyz.get(6..), Some("World"));// 索引不在字符边界上
    assert_eq!(xyz.get(20..25),None);// 索引超出范围
    let def = xyz.get_mut(..);// def 的类型是 Option<&mut str>
    let def = def.map(|c|{c.make_ascii_uppercase();&*c});// 这里c的类型是&mut str,必须用&*c变成&str
    assert_eq!(def,Some("HELLO WORLD"));
    assert_eq!(def.unwrap().to_lowercase(),"hello world".to_string());
}