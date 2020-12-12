pub struct S<T> {
    m: T,
}

pub struct D<T, U>(S<(T, U)>);

pub enum E {
    First(isize, f32),
    Second { m1: isize, m2: f32 },
    Third(D<isize, f32>),
    None,
}

impl From<E> for String {
    fn from(e: E) -> Self {
        // The following "match" helps us in deconstructing an enum entry to its
        // building blocks
        match e {
            E::First(i, f) => format!("First({}, {:.1})", i, f),
            E::Second { m1: i, m2: f } => format!("Second({}, {:.1})", i, f),
            E::Third(D(S { m: (i, f) })) => format!("Third({}, {:.1})", i, f),
            E::None => format!("None"),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::{D, E, S};
    use std::collections::HashMap;

    #[test]
    fn play_turbo_fish() {
        let v1 = (0..10).collect::<Vec<_>>();
        let v2: Vec<i32> = (0..10).collect();
        assert_eq!(v1, v2);
        let v3 = Vec::<u8>::with_capacity(1024);
        let v4: Vec<u8> = Vec::with_capacity(1024);
        assert_eq!(v3.capacity(), v4.capacity());
        assert_eq!(v3, v4);
    }

    #[test]
    fn play_tuple_struct() {
        let i = 5isize;
        let f = 4.3f32;

        let e1 = E::First(i, f);
        let e2 = E::Second { m1: i, m2: f };
        let e3 = E::Third(D(S { m: (i, f) }));

        assert_eq!(String::from(e1), String::from("First(5, 4.3)"));
        assert_eq!(String::from(e2), String::from("Second(5, 4.3)"));
        assert_eq!(String::from(e3), String::from("Third(5, 4.3)"));
    }

    #[test]
    fn play_rust_type() {
        //基本类型
        let a = 10;
        let b = 2;
        assert_eq!(a / b, 5);
        assert_eq!(a % b, 0);
        //数组 数组的元素类型必须一致 不支持关联数组
        let arr_a: [i32; 5] = [1, 2, 3, 4, 5];
        let arr_b = [1, 2, 3, 4, 5, 6, 1, 2, 3];
        assert_eq!(arr_b.len(), 9);
        assert_eq!(arr_a.split_at(2).0, [1, 2]);
        println!("{},{}", arr_a[0], arr_a.len());
        //元组 值和类型要一一对应
        let tup_a: (u8, String, char) = (255, String::from("abc"), 'x');
        assert_eq!(tup_a.0, 255);
        assert_eq!(tup_a.1, "abc");
        assert_eq!(tup_a.2, 'x');
        println!("{:?}", tup_a);
    }

    #[test]
    fn play_rust_map() {
        let a = vec![1, 2, 3];

        let m: HashMap<&i32, i32> = a
            .iter()
            .map(|i| (i, 1))
            .inspect(|i| println!("({}, {})", i.0, i.1))
            .collect();
        println!("{:?}", &m);

        let name = "Elahe Dastan";

        name.chars()
            .map(|b| (b, 1))
            .for_each(|i| println!("({}, {})", i.0, i.1));
    }
}
