// @see https://gist.github.com/sharpjs/31f83fa4f2e258bcd72a
fn fmap_vec(f: Box<dyn FnMut(i32) -> i32>, a: &Vec<i32>) -> Vec<i32> {
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

#[cfg(test)]
mod tests {
    use super::fmap_vec;
    #[test]
    fn fmap_vec_test() {
        fn twice(x: i32) -> i32 {
            x + x
        }
        assert_eq!(fmap_vec(Box::new(twice), &vec![1, 2, 3]), vec![2, 4, 6]);
    }
}

pub fn pure_vec(a:i32) -> Vec<i32>{
    vec![a]
}

pub fn apply_vec(f: Vec<Box<dyn FnMut(i32) -> i32>>,a: Vec<i32>) -> Vec<i32> {
    let f0 = f[0];
    fmap_vec(f0, &a)
}

pub fn either() {}
