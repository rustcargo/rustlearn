

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn play_iterator_consume() {
        assert_eq!(vec![1, 2, 3].iter().fold(0, |a, x| a + x), 6);
        let my_vec = vec![1, 2, 3];
        assert_eq!(my_vec.iter().sum::<i32>(), 6);
        let min = my_vec.iter().min().unwrap().clone();
        assert_eq!(min, 1);
        let max = my_vec.iter().max().unwrap().clone();
        assert_eq!(max, 3);
        let last = my_vec.iter().last().unwrap().clone();
        assert_eq!(last, 3);

        let index_value = my_vec.iter().nth(2).unwrap().clone();
        assert_eq!(index_value, 3)
    }

    #[test]
    fn play_iterator_operation() {
        let vec1 = vec![1, 2, 3];
        let vec2 = vec![3, 4, 5];
        vec1.iter().chain(vec2.iter())
            .inspect(|x| eprintln!("准备过滤元素: {}",x))
            .filter(|&x| x > &1)
            .inspect(|x| eprintln!("filter过滤后元素: {}",x))
            .skip(1)
            .inspect(|x| eprintln!("skip过滤后元素: {}",x))
            .take(3)
            .inspect(|x| eprintln!("take过滤后元素: {}",x)).sum::<i32>();
    }

    #[test]
    fn play_hashmap() {
        let mut scores: HashMap<String, i32> = HashMap::new();
        scores.insert(String::from("xiao ming"), 10);
        scores.insert(String::from("zhang san"), 20);

        for (key, value) in scores.iter() {

            eprintln!("{:?}", scores.get(key))
        }

        for (key, value) in scores.iter_mut() {

        }

//        let my_vec: Vec<(_)>  = scores.values().into_iter().collect();
//        eprintln!("{:?}", my_vec);

        for item in scores {
            eprintln!("{:?}", item)
        }
        // 这个语句错误: 前面的for语句移动了scores的所有权,scores已经无效
        // eprintln!("{:?}", scores)
    }
}