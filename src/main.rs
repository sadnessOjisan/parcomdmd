fn main() {
    let v = any_char("a".to_string());
    println!("{:?}", v);
}

fn any_char(input: String) -> Option<(String, String)> {
    let mut copy_string = input.clone();
    let first = copy_string.chars().nth(0);
    match first {
        Some(v) => {
            let str = v.to_string();
            Some((str, copy_string.split_off(1)))
        }
        None => None,
    }
}
