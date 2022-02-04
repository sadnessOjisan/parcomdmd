use std::any;

struct Input {
    pos: u32,
    text: String,
}

struct Ast {}

struct Parser {}

fn any_char(input: &str) -> Option<(String, String)> {
    let mut copy_string = input.clone().to_string();
    let first = copy_string.chars().nth(0);
    match first {
        Some(v) => {
            let str = v.to_string().as_str().to_owned();
            let a = copy_string.split_off(1);
            // &str を split_off するために String を経由させるの良くなさそう
            return Some((str, a));
        }
        None => None,
    }
}

// 条件を渡すとパーサーを作ってくれる
fn sat(f: impl Fn(&str) -> bool + 'static) -> Box<dyn Fn(&str) -> Option<(String, String)>> {
    let hoge: Box<dyn Fn(&str) -> Option<(String, String)>> =
        Box::new(move |input: &str| -> Option<(String, String)> {
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
    Box::new(hoge)
}

fn is_plus(input: &str) -> bool {
    input == "+"
}

fn is_factor(input: &str) -> bool {
    input == "*"
}

fn plus(input: &str) -> Option<(String, String)> {
    let plus = sat(is_plus);
    plus(input)
}

fn factor(input: &str) -> Option<(String, String)> {
    let plus = sat(is_factor);
    plus(input)
}

#[cfg(test)]
mod tests {
    use crate::parser::{any_char, plus};

    #[test]
    fn any_char_test() {
        let actual = any_char("test");
        assert_eq!(actual, Some(("t".to_string(), "est".to_string())));
    }

    #[test]
    fn any_char_test_empty() {
        let actual = any_char("");
        assert_eq!(actual, None);
    }

    #[test]
    fn any_char_test_single() {
        let actual = any_char("a");
        assert_eq!(actual, Some(("a".to_string(), "".to_string())));
    }

    #[test]
    fn plus_test() {
        let actual = plus("+");
        assert_eq!(actual, Some(("+".to_string(), "".to_string())));
    }

    #[test]
    fn plus_test_with_rest() {
        let actual = plus("+12");
        assert_eq!(actual, Some(("+".to_string(), "12".to_string())));
    }

    #[test]
    fn not_included_plus() {
        let actual = plus("12");
        assert_eq!(actual,None);
    }
}