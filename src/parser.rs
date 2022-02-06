use core::slice::SlicePattern;
use std::any;

struct Input {
    pos: u32,
    text: String,
}

struct Ast {}

struct Parser {}

// any_char, plus とかの parser を共通の型で縛りたい

fn any_char<'a>(input: &'a str) -> Option<(&'a str, &'a str)> {
    match &input.chars().nth(0) {
        Some(_) => {
            let a = &input[1..];
            return Some((&input[..1], a));
        }
        None => None,
    }
    // match &input.as_bytes() {
    //     [first, rest @ ..] => {
    //         Some((first, rest))
    //     }
    //     [] => {
    //         None
    //     }
    // }
}

// lto: link time optimization

// 条件を渡すとパーサーを作ってくれる
fn sat<'a>(
    f: impl Fn(&'a str) -> bool + 'static,
) -> Box<dyn Fn(&'a str) -> Option<(&'a str, &'a str)>> {
    let hoge: Box<dyn Fn(&'a str) -> Option<(&'a str, &'a str)>> =
        Box::new(move |input: &'a str| -> Option<(&'a str, &'a str)> {
            let item = any_char(input);
            match item {
                Some(v) => {
                    let parsed = v.0;
                    let is_ok = f(&parsed);
                    let rest = v.1;
                    let return_data = (parsed, rest);
                    if is_ok {
                        return Some(return_data);
                    }
                    None
                }
                None => None,
            }
        });
    hoge
}

fn is_num(input: &str) -> bool {
    input == "1"
        || input == "2"
        || input == "3"
        || input == "4"
        || input == "5"
        || input == "6"
        || input == "7"
        || input == "8"
        || input == "9"
        || input == "0"
}

fn is_plus(input: &str) -> bool {
    input == "+"
}

fn is_factor(input: &str) -> bool {
    input == "*"
}

fn plus(input: &str) -> Option<(&str, &str)> {
    let plus = sat(is_plus);
    plus(input)
}

fn factor(input: &str) -> Option<(&str, &str)> {
    let plus = sat(is_factor);
    plus(input)
}

fn num(input: &str) -> Option<(&str, &str)> {
    let plus = sat(is_num);
    plus(input)
}

// OCaml
//
// let many (p : 'a parser) : 'a list parser =
//  {
//    run =
//      (fun input ->
//        let xs = ref [] in
//        let rec loop input =
//          let input', result = p.run input in
//          match result with
//          | Ok x ->
//              xs := x :: !xs;
//              loop input'
//          | Error _ -> input
//        in
//        let input' = loop input in
//        (input', Ok (!xs |> List.rev)));
//  }

// parse many digit "3333a"
// ((many digit) "3333a") -> Some(("3333","a"))
fn many<'a>(parser: impl Fn(&'a str) -> Option<(&'a str, &'a str)>) {
    todo!("")
}

#[cfg(test)]
mod tests {
    use crate::parser::{any_char, plus};

    #[test]
    fn any_char_test() {
        let actual = any_char("test");
        assert_eq!(actual, Some(("t", "est")));
    }

    #[test]
    fn any_char_test_empty() {
        let actual = any_char("");
        assert_eq!(actual, None);
    }

    #[test]
    fn any_char_test_single() {
        let actual = any_char("a");
        assert_eq!(actual, Some(("a", "")));
    }

    #[test]
    fn plus_test() {
        let actual = plus("+");
        assert_eq!(actual, Some(("+", "")));
    }

    #[test]
    fn plus_test_with_rest() {
        let actual = plus("+12");
        assert_eq!(actual, Some(("+", "12")));
    }

    #[test]
    fn not_included_plus() {
        let actual = plus("12");
        assert_eq!(actual, None);
    }
}
