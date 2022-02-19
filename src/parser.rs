struct Nat {
    value: i64
}

enum Factor {
    Nat(Nat),
    Paren(Box<Expr>)
}


enum Term {
    Nat(Nat),
    Factor(Factor)
}

enum Expr {
    Plus(Box<Expr>, Box<Expr>),
    Term(Term)
}

struct Ast {
    expr: Expr
}

pub fn exec(input: &str) -> Ast {
    // どのようにして parser を呼び出して、AST を構築すべきか？
    // パーサの引数に渡して行って破壊的変更していく？
}


// any_char, plus とかの parser を共通の型で縛りたい

pub fn any_char(input: &str) -> Option<(char, &str)> {
    input.chars().next().map(|first| (first, &input[1..]))
}

// 条件を渡すとパーサーを作ってくれる
pub fn sat(pred: impl Fn(char) -> bool) -> impl FnOnce(&str) -> Option<(char, &str)> {
    move |input| -> Option<(char, &str)> {
        any_char(input).and_then(|(parsed, rest)| pred(parsed).then(|| (parsed, rest)))
    }
}

pub fn prefix(pr: char) -> impl FnOnce(&str) -> Option<(char, &str)> {
    let pred = move |input: char| -> bool { pr == input };
    sat(pred)
}


pub fn is_digit(input: char) -> bool {
    matches!(input, '0'..='9')
}

pub fn is_plus(input: char) -> bool {
    matches!(input, '+')
}

pub fn is_factor(input: char) -> bool {
    matches!(input, '*')
}

pub fn plus(input: &str) -> Option<(char, &str)> {
    let plus = sat(is_plus);
    plus(input)
}

pub fn factor(input: &str) -> Option<(char, &str)> {
    let plus = sat(is_factor);
    plus(input)
}

pub fn digit(input: &str) -> Option<(char, &str)> {
    let plus = sat(is_digit);
    plus(input)
}

// parse many digit "3333a"
// ((many digit) "3333a") -> Some(("3333","a"))
pub fn many(
    parser: impl Fn(&str) -> Option<(char, &str)>,
) -> impl FnOnce(&str) -> Option<(String, &str)> {
    move |input| {
        let mut result = String::new();
        result.reserve(input.len());
        let mut target = input;
        while let Some((accecpted, rest)) = parser(target) {
            result.push(accecpted);
            target = rest;
        }
        (!result.is_empty()).then(|| (result, target))
    }
}

// 左でパースした後に、その結果を入力に右でパーサーした結果を出力するパーサー
// let get_second = naive_left(any_char, any_char)
// get_second("abcd") // [b, cd]
pub fn discard_left(
    pA: impl Fn(&str) -> Option<(char, &str)>,
    pB: impl Fn(&str) -> Option<(char, &str)>,
) -> impl Fn(&str) -> Option<(char, &str)> {
    move |input| {
        let left_parsed = pA(input).and_then(|(parsed, rest)| pB(rest));
        left_parsed
    }
}

// 右でパースした後に、その結果を入力に右でパーサーした結果を出力するパーサー
pub fn naive_discard_right(
    pA: impl Fn(&str) -> Option<(char, &str)>,
    pB: impl Fn(&str) -> Option<(char, &str)>,
) -> impl Fn(&str) -> Option<(char, &str)> {
    move |input| {
        let left_parsed = pA(input).and_then(|(parsed, rest)| {
            let parsed2 = pB(rest);
            // Q: ここも and_then で書きたいが、parsed を使いたいのでかけない
            match parsed2 {
                Some(s2) => Some((parsed, s2.1)),
                None => None,
            }
        });
        left_parsed
    }
}

pub fn alternative(
    pA: impl Fn(&str) -> Option<(char, &str)>,
    pB: impl Fn(&str) -> Option<(char, &str)>,
) -> impl Fn(&str) -> Option<(char, &str)> {
    move |input| {
        let parsed = pA(input);
        match parsed {
            Some(p) => Some(p),
            None => pB(input),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn many_parse() {
        let many_parser = many(digit);
        let actual = many_parser("123a");
        assert_eq!(actual, Some(("123".to_string(), "a")));
    }

    #[test]
    fn naive_discard_left_test() {
        let left_parser = discard_left(any_char, any_char);
        let actual = left_parser("abcde");
        assert_eq!(actual, Some(('b', "cde")));
    }

    #[test]
    fn naive_discard_right_test() {
        let right_parser = naive_discard_right(any_char, any_char);
        let actual = right_parser("abcde");
        assert_eq!(actual, Some(('a', "cde")));
    }

    // let get_middle = get_char *> get_char <* get_char
    // let get_middle =right_parser(left_parser(get_char, get_char), get_char)
    #[test]
    fn middle() {
        let middle_parser = discard_left(any_char, naive_discard_right(any_char, any_char));
        let actual = middle_parser("abc");
        assert_eq!(actual, Some(('b', "")));
    }

    #[test]
    fn prefix_test() {
        let start_paren_parser = prefix('(');
        let actual = start_paren_parser("(1+2)*3");
        assert_eq!(actual, Some(('(', "1+2)*3")));
    }

    #[test]
    fn alternative_test() {
        let plus_or_digit_parser = alternative(digit, plus);
        let actual = plus_or_digit_parser("+2");
        assert_eq!(actual, Some(('+', "2")));
        let actual = plus_or_digit_parser("1+2");
        assert_eq!(actual, Some(('1', "+2")));
    }
}
