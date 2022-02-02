use std::any;

struct Input {
    pos: u32,
    text: String,
}

struct Ast {}

struct Parser {}

fn any_char(input: &str) -> Option<(&str, &str)> {
    let mut copy_string = input.clone();
    let first = copy_string.chars().nth(0);
    match first {
        Some(v) => {
            let str = v.to_string().as_str();
            // &str を split_off するために String を経由させるの良くなさそう
            Some((str, copy_string.to_string().split_off(1).as_str()))
        }
        None => None,
    }
}

// 条件を渡すとパーサーを作ってくれる
fn sat(f: impl Fn(&str) -> bool + 'static) -> Box<dyn Fn(&str) -> Option<(&str, &str)>> {
    let hoge: Box<dyn Fn(&str) -> Option<(&str, &str)>> =
        Box::new(move |input: &str| -> Option<(&str, &str)> {
            let item = any_char(input);
            match item {
                Some(v) => {
                    let parsed = &v.0;
                    let is_ok = f(parsed);
                    if is_ok {
                        return Some(v);
                    }
                    None
                }
                None => None,
            }
        });
    Box::new(hoge)
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

#[cfg(test)]
mod tests {
    use crate::parser::any_char;

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
}

// impl Applicative for Parser {
//     fn run(input: Input) -> Result<(Ast, Input), String>{}
// }
