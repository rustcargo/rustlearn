//首先看一下str 和 String之间的区别：String是一个可变的、堆上分配的UTF-8的字节缓冲区。
// 而str是一个不可变的固定长度的字符串，如果是从String解引用而来的，则指向堆上，如果是字面值，则指向静态内存。


#[cfg(test)]
mod tests {
    use std::ops::Deref;

    #[test]
    fn str_or_string() {
        let a = "hello";
        let b = String::from("hello");
        assert_eq!(a.len(), b.len());
        println!("{}", b.capacity());
        // &str更像一个固定的数组，String像一个可变的数组。
        // 如果只想要一个字符串的只读视图，或者&str作为一个函数的参数，那就首选&str。如果想拥有所有权，想修改字符串那就用String吧。

        let mut s = String::from("hello world");
        s.push_str(" ok");

        let cover_to_str = s.as_str();
        println!("{}", cover_to_str);

        //Deref和DerefMut都是Rust中的trait，用来对指针类型进行转化，得到指针所指向的内容。比如从Box<T>或Rc<T>中得到T，或是从String中得到&str。
        let cover_to_str2 = s.deref();
        println!("{}", cover_to_str2);
    }

    #[test]
    fn box_rc() {
        //& 在Rust里面称作borrow类型。只是引用内存上某个位置的值，并不拥有所指的值。如果我们想使用&来修复这个问题，我们需要注意一个问题就是生命周期，borrow类型生命周期取决owner的生命周期。
        let a = 25;
        let b = &a;
        assert_eq!(a, *b);

        //Box是一种智能指针，零运行时开销。拥有它所指向的数据。我们为什么称做智能的，是因为当执行过边界，它将drop掉它所指向的数据然后drop本身。不需要手动管理内存！！！
        let box_a = 25;
        let box_b = Box::new(box_a);
        assert_eq!(box_a, *box_b);
        //Rc是另外一种智能指针，是reference count的简称。用于记录数据结构被引用的次数。当引用的数字减小到0时，就自行清理。如果在一个线程，对于同一个数据有多个owner的时候，我们选用Rc。对于多线程，我们有Arc（atomic reference count)
        
    }
}