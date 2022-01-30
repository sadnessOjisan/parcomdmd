// @see https://gist.github.com/sharpjs/31f83fa4f2e258bcd72a
pub fn fmap() {
    let a = (0..).map(|n| n * n);
}

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

// OCaml
// let map (f : 'a -> 'b) (p : 'a parser) : 'b parser =
// {
//     run =
//       (fun input ->
//         match p.run input with
//         | input', Ok x -> (input', Ok (f x))
//         | input', Error error -> (input', Error error));
//   }

pub fn pure() {}

pub fn apply() {}

pub fn either() {}
