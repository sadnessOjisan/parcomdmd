pub fn get_moji(moji: String) -> Result<(String, String), String> {
    let first = moji.chars().nth(0);
    match first {
        Some(txt) => {
            let orig = String::from(txt);
            let (f, s) = orig.split_at(1);
            Ok((String::from(f), String::from(s)))
        }
        None => Err("空文字です".to_string()),
    }
}

struct Input {
    text: String,
    position: u32,
}

struct Parser {
    run: dyn Fn(
        Input,
    ) -> Result<
        (
            Input,
            String, // parse した残り
        ),
        String, // error message
    >,
}
