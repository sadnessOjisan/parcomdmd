use std::{any, vec};

struct Input {
    pos: u32,
    text: String,
}

struct Ast {}

struct Parser {}

// any_char, plus とかの parser を共通の型で縛りたい

fn any_char<'a>(input: &'a str) -> Option<(char, &'a str)> {
    match &input.chars().nth(0) {
        Some(_) => {
            let rest = &input[1..];
            let first = &input[..1];
            let fisrt_char = first.chars().nth(0).unwrap();
            return Some((fisrt_char, rest));
        }
        None => None,
    }
}

// lto: link time optimization

// 条件を渡すとパーサーを作ってくれる
fn sat<'a>(f: impl Fn(char) -> bool + 'static) -> Box<dyn Fn(&'a str) -> Option<(char, &'a str)>> {
    let parser: Box<dyn Fn(&'a str) -> Option<(char, &'a str)>> =
        Box::new(move |input: &'a str| -> Option<(char, &'a str)> {
            let item = any_char(input);
            match item {
                Some(v) => {
                    let parsed = v.0;
                    let is_ok = f(parsed);
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
    parser
}

fn is_num(input: char) -> bool {
    input == '1'
        || input == '2'
        || input == '3'
        || input == '4'
        || input == '5'
        || input == '6'
        || input == '7'
        || input == '8'
        || input == '9'
        || input == '0'
}

fn is_plus(input: char) -> bool {
    input == '+'
}

fn is_factor(input: char) -> bool {
    input == '*'
}

fn plus(input: &str) -> Option<(char, &str)> {
    let plus = sat(is_plus);
    plus(input)
}

fn factor(input: &str) -> Option<(char, &str)> {
    let plus = sat(is_factor);
    plus(input)
}

fn num(input: &str) -> Option<(char, &str)> {
    let plus = sat(is_num);
    plus(input)
}

// parse many digit "3333a"
// ((many digit) "3333a") -> Some(("3333","a"))
fn many<'a>(
    parser: impl Fn(&'a str) -> Option<(char, &'a str)>,
) -> Box<dyn FnOnce(&'a str) -> Option<(Vec<char>, &'a str)>> {
    let mut result = (vec![], "");
    let p = Box::new(|x| {
        let xc = x.clone();
        result.1 = xc;
        loop {
            let parsed = parser(x);
            match parsed {
                Some(v) => {
                    result.0.push(v.0);
                    result.1 = v.1;
                    x = v.1;
                }
                None => break,
            }
        }
        if (result.0.len() == 0) {
            None
        } else {
            Some(result)
        }
    });
    p
}

#[cfg(test)]
mod tests {
    use crate::parser::{any_char, plus};

    use super::{many, num};

    #[test]
    fn any_char_test() {
        let actual = any_char("test");
        assert_eq!(actual, Some(('t', "est")));
    }

    #[test]
    fn any_char_test_empty() {
        let actual = any_char("");
        assert_eq!(actual, None);
    }

    #[test]
    fn any_char_test_single() {
        let actual = any_char("a");
        assert_eq!(actual, Some(('a', "")));
    }

    #[test]
    fn plus_test() {
        let actual = plus("+");
        assert_eq!(actual, Some(('+', "")));
    }

    #[test]
    fn plus_test_with_rest() {
        let actual = plus("+12");
        assert_eq!(actual, Some(('+', "12")));
    }

    #[test]
    fn not_included_plus() {
        let actual = plus("12");
        assert_eq!(actual, None);
    }

    fn many_parse() {
        let many_parser = many(num);
        let actual = many_parser("123a");
        assert_eq!(actual, Some((vec!['1', '2', '3'], "a")));
    }
}
