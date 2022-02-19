#[allow(dead_code)]
struct Input {
    pos: u32,
    text: String,
}

#[allow(dead_code)]
struct Ast {}

#[allow(dead_code)]
struct Parser {}

// any_char, plus とかの parser を共通の型で縛りたい

#[allow(dead_code)]
fn any_char(input: &str) -> Option<(char, &str)> {
    input.chars().next().map(|first| (first, &input[1..]))
}

// 条件を渡すとパーサーを作ってくれる
#[allow(dead_code)]
fn sat(
    pred: impl Fn(char) -> bool,
) -> impl FnOnce(&str) -> Option<(char, &str)> {
    move |input| -> Option<(char, &str)> {
        any_char(input).and_then(|(parsed, rest)| pred(parsed).then(|| (parsed, rest)))
    }
}

#[allow(dead_code)]
fn is_num(input: char) -> bool {
    matches!(input, '0'..='9')
}

#[allow(dead_code)]
fn is_plus(input: char) -> bool {
    matches!(input, '+')
}

#[allow(dead_code)]
fn is_factor(input: char) -> bool {
    matches!(input, '*')
}

#[allow(dead_code)]
fn plus(input: &str) -> Option<(char, &str)> {
    let plus = sat(is_plus);
    plus(input)
}

#[allow(dead_code)]
fn factor(input: &str) -> Option<(char, &str)> {
    let plus = sat(is_factor);
    plus(input)
}

#[allow(dead_code)]
fn num(input: &str) -> Option<(char, &str)> {
    let plus = sat(is_num);
    plus(input)
}


// parse many digit "3333a"
// ((many digit) "3333a") -> Some(("3333","a"))
#[allow(dead_code)]
fn many(
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
fn discard_left(pA: impl Fn(&str) -> Option<(char, &str)>, pB: impl Fn(&str) -> Option<(char, &str)>)-> impl Fn(&str) -> Option<(char, &str)>{
    move |input| {
        let left_parsed = pA(input).and_then(|(parsed, rest)|{
            pB(rest)
        });
        left_parsed
    }
}

// 右でパースした後に、その結果を入力に右でパーサーした結果を出力するパーサー
fn naive_discard_right(pA: impl Fn(&str) -> Option<(char, &str)>, pB: impl Fn(&str) -> Option<(char, &str)>)-> impl Fn(&str) -> Option<(char, &str)>{
    move |input| {
        let left_parsed = pA(input).and_then(|(parsed, rest)|{
            let parsed2 = pB(rest);
            // Q: ここも and_then で書きたいが、parsed を使いたいのでかけない
            match parsed2 {
                    Some(s2) => {
                        Some((parsed, s2.1))
                    }
                    None => {
                        None
                    }
                }
        });
        left_parsed
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
        let many_parser = many(num);
        let actual = many_parser("123a");
        assert_eq!(actual, Some(("123".to_string(), "a")));
    }

    #[test]
    fn naive_discard_left_test(){
        let left_parser = discard_left(any_char, any_char);
        let actual = left_parser("abcde");
        assert_eq!(actual, Some(('b', "cde")));
    }

    #[test]
    fn naive_discard_right_test(){
        let right_parser = naive_discard_right(any_char, any_char);
        let actual = right_parser("abcde");
        assert_eq!(actual, Some(('a', "cde")));
    }

    // let get_middle = get_char *> get_char <* get_char
    // let get_middle =right_parser(left_parser(get_char, get_char), get_char)
    #[test]
    fn middle(){
        let middle_parser = discard_left(any_char, naive_discard_right(any_char, any_char));
        let actual = middle_parser("abc");
        assert_eq!(actual, Some(('b', "")));
    }
}