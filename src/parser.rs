struct Input {
    pos: u32,
    text: String,
}


struct Ast {}

struct Parser {}

// fn digit(input: Input) -> Option<i32>{}

fn any_char(input: String) -> Option<(String, String)> {
    let mut copy_string = input.clone();
    let first = copy_string.chars().nth(0);
    match first {
        Some(v) => {
            let str = v.to_string();
            Some((str, copy_string.split_off(1)))
        }
        None => None
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::any_char;

    #[test]
    fn any_char_test() {
        let actual = any_char("test".to_string());
        assert_eq!(actual, Some(("t".to_string(), "est".to_string())));
    }

    #[test]
    fn any_char_test_empty() {
        let actual = any_char("".to_string());
        assert_eq!(actual, None);
    }

    #[test]
    fn any_char_test_single() {
        let actual = any_char("a".to_string());
        assert_eq!(actual, Some(("a".to_string(), "".to_string())));
    }
}

// impl Applicative for Parser {
//     fn run(input: Input) -> Result<(Ast, Input), String>{}
// }
