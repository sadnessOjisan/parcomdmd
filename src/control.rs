// @see https://gist.github.com/sharpjs/31f83fa4f2e258bcd72a
pub fn fmap (){}

// OCaml
// let map (f : 'a -> 'b) (p : 'a parser) : 'b parser =
// {
//     run =
//       (fun input ->
//         match p.run input with
//         | input', Ok x -> (input', Ok (f x))
//         | input', Error error -> (input', Error error));
//   }
fn map(f: Box<dyn Fn(i32) -> i32>, a: i32) -> i32{
    3
}

pub fn pure(){}

pub fn apply(){}

pub fn either(){}