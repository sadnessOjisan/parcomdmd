pub mod vec {
    // 高階関数の引数って参照にしたら何か問題あるっけ？？
    pub fn fmap(f: Box<dyn Fn(i32) -> i32>, a: &Vec<i32>) -> Vec<i32> {
        let mut b = vec![];
        for &num in a {
            b.push(f(num));
        }
        b

        // 本当はこうしたい
        // Box<dyn FnMut(i32) -> i32> を map に入れられるようにするためにはどうすればいいのだろうか？
        // iterator.Map は sized を要求しているのえ、Boxに写した後だともう無理？
        // a.iter().map(f).collect::<Vec<_>>();
    }

    pub fn pure(a: i32) -> Vec<i32> {
        vec![a]
    }

    pub fn apply_1(f: Vec<Box<dyn Fn(i32) -> i32>>, a: &Vec<i32>) -> Vec<i32> {
        let f0 = &f[0];
        let mut b = vec![];
        for &v in a {
            b.push(f0(v));
        }
        b
    }

    pub fn either() {}

    #[cfg(test)]
    mod tests {
        use crate::control::vec::apply_1;

        use super::fmap;
        #[test]
        fn fmap_test() {
            fn twice(x: i32) -> i32 {
                x + x
            }
            assert_eq!(fmap(Box::new(twice), &vec![1, 2, 3]), vec![2, 4, 6]);
        }
        #[test]
        fn apply_test() {
            fn twice(x: i32) -> i32 {
                x + x
            }
            assert_eq!(
                apply_1(vec![Box::new(twice)], &vec![1, 2, 3]),
                vec![2, 4, 6]
            );
        }
    }
}

pub mod option {
    pub fn fmap(f: Box<dyn Fn(i32) -> i32>, a: &Option<i32>) -> Option<i32> {
        match a {
            Some(v) => {
                // clone がないと動かない。i32は自動でcopyしてくれるのでは？
                Some(f(v.clone()))
            }
            None => None,
        }
    }

    pub fn pure(a: i32) -> Option<i32> {
        Some(a)
    }

    pub fn apply_1(f: Option<Box<dyn Fn(i32) -> i32>>, a: &Option<i32>) -> Option<i32> {
        match f {
            Some(f) => match a {
                Some(v) => Some(f(v.clone())),
                None => None,
            },
            None => None,
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::control::option::apply_1;

        use super::fmap;
        #[test]
        fn fmap_test() {
            fn twice(x: i32) -> i32 {
                x + x
            }
            assert_eq!(fmap(Box::new(twice), &Some(1)), Some(2));
        }

        #[test]
        fn apply_test() {
            assert_eq!(apply_1(Some(Box::new(|x| { x + 1 })), &Some(1)), Some(2));
        }
    }
}
