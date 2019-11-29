//宏就是以macro_rules开头的定义的

/*
ident: 标识符，用来表示函数或变量名
expr: 表达式
block: 代码块，用花括号包起来的多个语句
pat: 模式，普通模式匹配（非宏本身的模式）中的模式，例如 Some(t), (3, 'a', _)
path: 路径，注意这里不是操作系统中的文件路径，而是用双冒号分隔的限定名(qualified name)，如 std::cmp::PartialOrd
tt: 单个语法树
ty: 类型，语义层面的类型，如 i32, char
item: 条目，
meta: 元条目
stmt: 单条语句，如 let a = 42;
*/

//匹配方式为()闭合，$func_name捕捉名字，ident代表标识符
macro_rules! create_function {
    ($func_name:ident) => (
        fn $func_name() {
            eprintln!("function {:?} is called", stringify!($func_name))
        }
    )
}

//带重复模式的宏定义，注意有两层$，第一层代表每个重复的模式，第二层代表一个重复中如何匹配。
macro_rules! vector{
	($($x:expr),*)=>{
		{
			let mut temp_vec = Vec::new();
			//生成的时候，也是如此
			//第一层$代表如何生成每个重复的模式
			//第二层$代表如何生成一个重复中的内容
			$(temp_vec.push($x);)*
			temp_vec
		}
	};
}

macro_rules! find_min{
	($x:expr)=>($x);
	($x:expr,$($y:expr),+)=>{
		std::cmp::min($x,find_min!($($y),+))
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn play_create_fn() {
        create_function!(hello_macro);
        hello_macro();
    }

    #[test]
    fn play_vector() {
        let a = vector![1,2,3,4];
        eprintln!("{:?}",a);
    }

    #[test]
    fn play_find_min() {
        eprintln!("{}",find_min!(23));
        eprintln!("{}",find_min!(23,2));
        eprintln!("{}",find_min!(23,2,43));
    }
}
