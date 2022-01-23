// @see https://gist.github.com/sharpjs/31f83fa4f2e258bcd72a
pub fn fmap() {}

// OCaml
// let map (f : 'a -> 'b) (p : 'a parser) : 'b parser =
// {
//     run =
//       (fun input ->
//         match p.run input with
//         | input', Ok x -> (input', Ok (f x))
//         | input', Error error -> (input', Error error));
//   }
fn map(f: Box<dyn Fn(i32) -> i32>, a: i32) -> i32 {
    3
}

// haskell
// class Functor f where
//   fmap :: (a -> b) -> f a -> f b
pub trait Functor<T, U> {
    type Out;

    fn fmap<F: Fn(&T) -> U>(&self, F) -> Self::Out;
}

// Arity 1
impl<T, U> Functor<T, U> for (T,)  {
    type Out = (U,);

    #[inline(always)]
    fn fmap<F: Fn(&T) -> U>(&self, f: F) -> Self::Out {
        (f(&self.0),)
    }
}


// functor の下に作りたい (どうやって fmap をつかえばいい？)
// :trait でよさそう

// @see https://qiita.com/emo/items/bda7ea17bdec9555782d
pub trait Apprecative:Functor {
    fn pure(){}

    // 演算子定義がないから呼び出すときはひたすら左結合？
    fn apply(){}
}


pub fn pure() {}

pub fn apply() {}

pub fn either() {}
