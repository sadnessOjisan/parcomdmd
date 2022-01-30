// // こういう関数の型と制約を作りたい。後述のparse関数の引数に入れたい
// // parser を引数に取った方が良さそう
// pub fn get_moji(moji: String) -> Result<(String, String), String> {
//     let first = moji.chars().nth(0);
//     match first {
//         // head と tail で分けられないんだっけ
//         Some(txt) => {
//             let orig = String::from(txt);
//             let (f, s) = orig.split_at(1);
//             Ok((String::from(f), String::from(s)))
//         }
//         None => Err("空文字です".to_string()),
//     }
// }

// pub fn parse_md() {
//     // parse block
//     // parse inline
//     // parse block
//     // parse head
//     // parse paragraph
//     // ...
//     // parse inline
//     // parse span
//     // parse text
//     // ...
// }

// pub fn parse(parser: Parser) {
//     parser.run()
// }

// struct Input {
//     text: String,
//     position: u32,
// }

// // trait の方が良かった？
// struct Parser {
//     run: dyn Fn(
//         Input,
//     ) -> Result<
//         (
//             Input,
//             String, // parse した残り
//         ),
//         String, // error message
//     >,
// }
